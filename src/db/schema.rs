// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "action_type_enum"))]
    pub struct ActionTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "event_type_enum"))]
    pub struct EventTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "field_type_enum"))]
    pub struct FieldTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "field_value_enum"))]
    pub struct FieldValueEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "query_type_enum"))]
    pub struct QueryTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "recipe_template_type_enum"))]
    pub struct RecipeTemplateTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resource_type_enum"))]
    pub struct ResourceTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_type_enum"))]
    pub struct RoleTypeEnum;
}

diesel::table! {
    use diesel::sql_types::*;

    agents (id) {
        id -> Uuid,
        name -> Text,
        note -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    economic_resources (id) {
        id -> Uuid,
        resource_specification_id -> Uuid,
        name -> Text,
        note -> Nullable<Text>,
        accounting_quantity -> Int4,
        on_hand_quantity -> Int4,
        tracking_identifier -> Nullable<Text>,
        current_location -> Text,
        lot -> Nullable<Text>,
        contained_in -> Nullable<Uuid>,
        created_at -> Timestamp,
        reference_number -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    process_events (id) {
        id -> Uuid,
        process_flow_data_field_id -> Uuid,
        query_value -> Nullable<Uuid>,
        value -> Text,
        reference_number -> Int4,
        provider -> Uuid,
        receiver -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EventTypeEnum;
    use super::sql_types::ActionTypeEnum;
    use super::sql_types::RoleTypeEnum;

    process_flow (id) {
        id -> Uuid,
        process_id -> Uuid,
        event_type -> EventTypeEnum,
        action_type -> ActionTypeEnum,
        role -> RoleTypeEnum,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QueryTypeEnum;

    process_flow_data_field_queries (id) {
        id -> Uuid,
        query_type -> QueryTypeEnum,
        table_name -> Text,
        fields -> Jsonb,
        conditions -> Nullable<Jsonb>,
        additional_clauses -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FieldValueEnum;
    use super::sql_types::FieldTypeEnum;

    process_flow_data_fields (id) {
        id -> Uuid,
        process_flow_id -> Uuid,
        field_value -> FieldValueEnum,
        field -> Text,
        field_type -> FieldTypeEnum,
        note -> Nullable<Text>,
        required -> Bool,
        query -> Nullable<Uuid>,
        default_value -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    processes (id) {
        id -> Uuid,
        recipe_template_id -> Uuid,
        recipe_id -> Uuid,
        name -> Text,
        note -> Nullable<Text>,
        output_of -> Nullable<Uuid>,
        template_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FieldValueEnum;
    use super::sql_types::FieldTypeEnum;

    recipe_flow_template_data_fields (id) {
        id -> Uuid,
        recipe_flow_template_id -> Uuid,
        field_value -> FieldValueEnum,
        field -> Text,
        field_type -> FieldTypeEnum,
        note -> Nullable<Text>,
        required -> Bool,
        default_value -> Nullable<Text>,
        query -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EventTypeEnum;
    use super::sql_types::RoleTypeEnum;
    use super::sql_types::ActionTypeEnum;

    recipe_flow_templates (id) {
        id -> Uuid,
        recipe_template_id -> Uuid,
        event_type -> EventTypeEnum,
        role_type -> RoleTypeEnum,
        action -> ActionTypeEnum,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    recipe_resources (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        resource_specification_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RecipeTemplateTypeEnum;

    recipe_templates (id) {
        id -> Uuid,
        name -> Text,
        recipe_template_type -> RecipeTemplateTypeEnum,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    recipe_templates_access (id) {
        id -> Uuid,
        agent_id -> Uuid,
        recipe_template_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    recipes (id) {
        id -> Uuid,
        agent_id -> Uuid,
        name -> Text,
        note -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ResourceTypeEnum;

    resource_specifications (id) {
        id -> Uuid,
        agent_id -> Uuid,
        name -> Text,
        note -> Nullable<Text>,
        created_at -> Timestamp,
        resource_type -> ResourceTypeEnum,
        unit_of_measure -> Text,
    }
}

diesel::joinable!(economic_resources -> resource_specifications (resource_specification_id));
diesel::joinable!(process_events -> process_flow_data_fields (process_flow_data_field_id));
diesel::joinable!(process_flow -> processes (process_id));
diesel::joinable!(process_flow_data_fields -> process_flow (process_flow_id));
diesel::joinable!(process_flow_data_fields -> process_flow_data_field_queries (query));
diesel::joinable!(processes -> recipes (recipe_id));
diesel::joinable!(recipe_flow_template_data_fields -> process_flow_data_field_queries (query));
diesel::joinable!(recipe_flow_template_data_fields -> recipe_flow_templates (recipe_flow_template_id));
diesel::joinable!(recipe_flow_templates -> recipe_templates (recipe_template_id));
diesel::joinable!(recipe_resources -> recipes (recipe_id));
diesel::joinable!(recipe_resources -> resource_specifications (resource_specification_id));
diesel::joinable!(recipe_templates_access -> agents (agent_id));
diesel::joinable!(recipe_templates_access -> recipe_templates (recipe_template_id));
diesel::joinable!(recipes -> agents (agent_id));
diesel::joinable!(resource_specifications -> agents (agent_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    economic_resources,
    process_events,
    process_flow,
    process_flow_data_field_queries,
    process_flow_data_fields,
    processes,
    recipe_flow_template_data_fields,
    recipe_flow_templates,
    recipe_resources,
    recipe_templates,
    recipe_templates_access,
    recipes,
    resource_specifications,
);
