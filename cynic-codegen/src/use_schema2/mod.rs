//mod model;
mod params;

pub use params::UseSchemaParams;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::parse_quote;

use crate::{
    field_type::FieldType,
    ident::Ident,
    schema::{self, SchemaLoadError},
    type_index::TypeIndex,
};

pub fn use_schema(input: UseSchemaParams) -> Result<TokenStream, SchemaLoadError> {
    use quote::{quote, TokenStreamExt};

    let document = crate::schema::load_schema(input.schema_filename)?;

    let mut output = TokenStream::new();

    // TODO: Refactor this so it's not just one big loop
    for definition in document
        .definitions
        .into_iter()
        .filter_map(type_def_from_definition)
    {
        match definition {
            graphql_parser::schema::TypeDefinition::Scalar(def) => {
                let ident = Ident::for_type(&def.name);
                output.append_all(quote! {
                    pub struct #ident {}
                });
            }
            graphql_parser::schema::TypeDefinition::Object(def) => {
                let object_marker_type_name = Ident::for_type(&def.name);
                output.append_all(quote! {
                    pub struct #object_marker_type_name;
                });

                let mut field_module_contents = Vec::new();
                for field in def.fields {
                    // TODO: wonder if we need a different Ident func for this.
                    // Technically field.name is a field, but we're only using it
                    // as a type.
                    let field_marker_type_name = Ident::for_type(&field.name);
                    let field_name_literal = proc_macro2::Literal::string(&field.name);

                    // Note: See if we can get rid of TypeIndex from this
                    // call once done.  That'd be nice.
                    let field_type =
                        FieldType::from_schema_type(&field.field_type, &TypeIndex::empty());
                    let field_type_marker = field_type.to_tokens(None, parse_quote! { super });

                    field_module_contents.push(quote! {
                        pub struct #field_marker_type_name;

                        impl ::cynic::core::FieldName for #field_marker_type_name {
                            fn name() -> &'static str {
                                #field_name_literal
                            }
                        }

                        impl ::cynic::core::HasField<#field_marker_type_name, #field_type_marker> for super::#object_marker_type_name {}

                        // TODO: implement HasField for all the valid conversions...
                        // assuming that's even possible - implementing the deserialize might be tricky for
                        // some of them?  Need to check what's even allowed here...
                    })

                    // TODO: Handle arguments
                }

                let field_module_name = format_ident!("{}_fields", def.name.to_snake_case());
                output.append_all(quote! {
                    pub mod #field_module_name {
                        #(#field_module_contents)*
                    }
                });
            }
            graphql_parser::schema::TypeDefinition::Interface(def) => {
                let ident = Ident::for_type(&def.name);
                output.append_all(quote! {
                    pub struct #ident {}
                });

                // TODO: the rest of this.
            }
            graphql_parser::schema::TypeDefinition::Union(def) => {
                let ident = Ident::for_type(&def.name);
                output.append_all(quote! {
                    pub struct #ident {}
                });

                // TODO: the rest of this.
            }
            graphql_parser::schema::TypeDefinition::Enum(def) => {
                let ident = Ident::for_type(&def.name);
                output.append_all(quote! {
                    pub struct #ident {}
                });
            }
            graphql_parser::schema::TypeDefinition::InputObject(def) => {
                let ident = Ident::for_type(&def.name);
                output.append_all(quote! {
                    pub struct #ident {}
                });

                // TODO: Handle fields
            }
        }
    }

    // TODO: output defs for the built in scalars so we don't have
    // to special case them elsewhere...

    Ok(output)
}

fn type_def_from_definition(definition: schema::Definition) -> Option<schema::TypeDefinition> {
    match definition {
        graphql_parser::schema::Definition::TypeDefinition(inner) => Some(inner),
        _ => None,
    }
}