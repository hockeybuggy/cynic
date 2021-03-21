pub use queries::*;

#[cynic::schema_for_derives(file = r#"src/schema.graphql"#, module = "schema")]
mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct IntrospectionQuery {
        pub __schema: __Schema,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct __Schema {
        pub query_type: __Type,
        pub mutation_type: Option<__Type>,
        pub subscription_type: Option<__Type>,
        pub types: Vec<__Type2>,
        pub directives: Vec<__Directive>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct __Directive {
        pub name: String,
        pub description: Option<String>,
        pub args: Vec<__InputValue>,
        pub locations: Vec<__DirectiveLocation>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct __Type2 {
        pub kind: __TypeKind,
        pub name: Option<String>,
        pub description: Option<String>,
        #[arguments(include_deprecated = true)]
        pub fields: Option<Vec<__Field>>,
        pub input_fields: Option<Vec<__InputValue>>,
        pub interfaces: Option<Vec<__Type3>>,
        #[arguments(include_deprecated = true)]
        pub enum_values: Option<Vec<__EnumValue>>,
        pub possible_types: Option<Vec<__Type3>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct __EnumValue {
        pub name: String,
        pub description: Option<String>,
        pub is_deprecated: bool,
        pub deprecation_reason: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct __Field {
        pub name: String,
        pub description: Option<String>,
        pub args: Vec<__InputValue>,
        #[cynic(rename = "type")]
        pub type_: __Type3,
        pub is_deprecated: bool,
        pub deprecation_reason: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct __InputValue {
        pub name: String,
        pub description: Option<String>,
        #[cynic(rename = "type")]
        pub type_: __Type3,
        pub default_value: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct __Type3 {
        pub kind: __TypeKind,
        pub name: Option<String>,
        pub of_type: Option<__Type4>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct __Type4 {
        pub kind: __TypeKind,
        pub name: Option<String>,
        pub of_type: Option<__Type5>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct __Type5 {
        pub kind: __TypeKind,
        pub name: Option<String>,
        pub of_type: Option<__Type6>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "__Type")]
    pub struct __Type6 {
        pub kind: __TypeKind,
        pub name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct __Type {
        pub name: Option<String>,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(graphql_type = "__DirectiveLocation")]
    pub enum DirectiveLocation {
        Query,
        Mutation,
        Subscription,
        Field,
        FragmentDefinition,
        FragmentSpread,
        InlineFragment,
        VariableDefinition,
        Schema,
        Scalar,
        Object,
        FieldDefinition,
        ArgumentDefinition,
        Interface,
        Union,
        Enum,
        EnumValue,
        InputObject,
        InputFieldDefinition,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    #[cynic(graphql_type = "__TypeKind")]
    pub enum TypeKind {
        Scalar,
        Object,
        Interface,
        Union,
        Enum,
        InputObject,
        List,
        NonNull,
    }
}

mod schema {
    cynic::use_schema!(r#"src/schema.graphql"#);
}
