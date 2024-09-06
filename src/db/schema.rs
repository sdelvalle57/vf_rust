// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "action_type_enum"))]
    pub struct ActionTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "event_type_enum"))]
    pub struct EventTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "field_class_enum"))]
    pub struct FieldClassEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "field_type_enum"))]
    pub struct FieldTypeEnum;

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

    locations (id) {
        id -> Uuid,
        agent_id -> Uuid,
        name -> Text,
        value -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    lot_codes (id) {
        id -> Uuid,
        agent_id -> Uuid,
        current_lot_code -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FieldClassEnum;
    use super::sql_types::FieldTypeEnum;

    recipe_flow_template_data_fields (id) {
        id -> Uuid,
        recipe_flow_template_id -> Uuid,
        field_class -> FieldClassEnum,
        field -> Text,
        field_type -> FieldTypeEnum,
        note -> Nullable<Text>,
        required -> Bool,
        field_identifier -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    recipe_flow_template_visibility_fields (id) {
        id -> Uuid,
        recipe_flow_template_id -> Uuid,
        field_id -> Uuid,
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
    use super::sql_types::EventTypeEnum;
    use super::sql_types::RoleTypeEnum;
    use super::sql_types::ActionTypeEnum;

    recipe_process_flows (id) {
        id -> Uuid,
        recipe_process_id -> Uuid,
        recipe_flow_template_id -> Uuid,
        event_type -> EventTypeEnum,
        role_type -> RoleTypeEnum,
        action -> ActionTypeEnum,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RecipeTemplateTypeEnum;

    recipe_processes (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        recipe_template_id -> Nullable<Uuid>,
        name -> Text,
        recipe_type -> RecipeTemplateTypeEnum,
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
diesel::joinable!(locations -> agents (agent_id));
diesel::joinable!(lot_codes -> agents (agent_id));
diesel::joinable!(recipe_flow_template_data_fields -> recipe_flow_templates (recipe_flow_template_id));
diesel::joinable!(recipe_flow_template_visibility_fields -> recipe_flow_template_data_fields (field_id));
diesel::joinable!(recipe_flow_template_visibility_fields -> recipe_flow_templates (recipe_flow_template_id));
diesel::joinable!(recipe_flow_templates -> recipe_templates (recipe_template_id));
diesel::joinable!(recipe_process_flows -> recipe_flow_templates (recipe_flow_template_id));
diesel::joinable!(recipe_process_flows -> recipe_processes (recipe_process_id));
diesel::joinable!(recipe_processes -> recipe_templates (recipe_template_id));
diesel::joinable!(recipe_processes -> recipes (recipe_id));
diesel::joinable!(recipe_resources -> recipes (recipe_id));
diesel::joinable!(recipe_resources -> resource_specifications (resource_specification_id));
diesel::joinable!(recipe_templates_access -> agents (agent_id));
diesel::joinable!(recipe_templates_access -> recipe_templates (recipe_template_id));
diesel::joinable!(recipes -> agents (agent_id));
diesel::joinable!(resource_specifications -> agents (agent_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    economic_resources,
    locations,
    lot_codes,
    recipe_flow_template_data_fields,
    recipe_flow_template_visibility_fields,
    recipe_flow_templates,
    recipe_process_flows,
    recipe_processes,
    recipe_resources,
    recipe_templates,
    recipe_templates_access,
    recipes,
    resource_specifications,
);
