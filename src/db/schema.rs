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
    #[diesel(postgres_type(name = "field_group_class_enum"))]
    pub struct FieldGroupClassEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "field_type_enum"))]
    pub struct FieldTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "flow_through_enum"))]
    pub struct FlowThroughEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resource_type_enum"))]
    pub struct ResourceTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "restriction_enum"))]
    pub struct RestrictionEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_type_enum"))]
    pub struct RoleTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "template_type_enum"))]
    pub struct TemplateTypeEnum;
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

    counters (id) {
        id -> Uuid,
        agent_id -> Uuid,
        lot_code -> Int4,
        reference_number -> Int4,
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
    use super::sql_types::TemplateTypeEnum;

    map_templates (id) {
        id -> Uuid,
        name -> Text,
        #[sql_name = "type"]
        type_ -> TemplateTypeEnum,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    process_execution_custom_values (id) {
        id -> Uuid,
        process_execution_id -> Uuid,
        field_id -> Uuid,
        field_value -> Text,
        corrects -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActionTypeEnum;
    use super::sql_types::RoleTypeEnum;

    process_executions (id) {
        id -> Uuid,
        process_flow_id -> Uuid,
        action -> ActionTypeEnum,
        role_type -> RoleTypeEnum,
        resource_specification -> Nullable<Uuid>,
        resource_reference_number -> Nullable<Int4>,
        resource_lot_number -> Nullable<Int4>,
        resource_quantity -> Nullable<Int4>,
        to_resource_specification -> Nullable<Uuid>,
        to_resource_reference_number -> Nullable<Int4>,
        to_resource_lot_number -> Nullable<Int4>,
        provider_agent -> Uuid,
        receiver_agent -> Uuid,
        at_location -> Nullable<Uuid>,
        to_location -> Nullable<Uuid>,
        has_point_in_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        corrects -> Nullable<Uuid>,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FieldClassEnum;
    use super::sql_types::FieldTypeEnum;
    use super::sql_types::FlowThroughEnum;

    recipe_flow_template_data_fields (id) {
        id -> Uuid,
        recipe_flow_template_id -> Uuid,
        group_id -> Nullable<Uuid>,
        field_identifier -> Text,
        field_class -> FieldClassEnum,
        field -> Text,
        field_type -> FieldTypeEnum,
        note -> Nullable<Text>,
        required -> Bool,
        flow_through -> Nullable<FlowThroughEnum>,
        inherits -> Nullable<Uuid>,
        accept_default -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FieldGroupClassEnum;

    recipe_flow_template_group_data_fields (id) {
        id -> Uuid,
        name -> Text,
        group_class -> FieldGroupClassEnum,
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
        identifier -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FieldClassEnum;
    use super::sql_types::FieldTypeEnum;
    use super::sql_types::FlowThroughEnum;

    recipe_process_flow_data_fields (id) {
        id -> Uuid,
        recipe_process_flow_id -> Uuid,
        recipe_flow_template_data_field_id -> Nullable<Uuid>,
        group_id -> Nullable<Uuid>,
        field_identifier -> Text,
        field_class -> FieldClassEnum,
        field -> Text,
        field_type -> FieldTypeEnum,
        note -> Nullable<Text>,
        required -> Bool,
        default_value -> Nullable<Text>,
        flow_through -> Nullable<FlowThroughEnum>,
        inherits -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FieldGroupClassEnum;

    recipe_process_flow_group_data_fields (id) {
        id -> Uuid,
        name -> Text,
        group_class -> FieldGroupClassEnum,
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
        identifier -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    recipe_process_relations (id) {
        id -> Uuid,
        recipe_process_id -> Uuid,
        output_of -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActionTypeEnum;

    recipe_processes (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        recipe_template_id -> Nullable<Uuid>,
        name -> Text,
        commitment -> Nullable<ActionTypeEnum>,
        fulfills -> Nullable<Uuid>,
        identifier -> Text,
        trigger -> Nullable<ActionTypeEnum>,
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
    use super::sql_types::RestrictionEnum;

    recipe_template_blacklists (id) {
        id -> Int4,
        map_template_id -> Uuid,
        recipe_template_id -> Uuid,
        restricted_recipe_template_id -> Uuid,
        restriction_type -> RestrictionEnum,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActionTypeEnum;

    recipe_templates (id) {
        id -> Uuid,
        map_template_id -> Uuid,
        identifier -> Text,
        name -> Text,
        commitment -> Nullable<ActionTypeEnum>,
        fulfills -> Nullable<Uuid>,
        trigger -> Nullable<ActionTypeEnum>,
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

diesel::joinable!(counters -> agents (agent_id));
diesel::joinable!(economic_resources -> resource_specifications (resource_specification_id));
diesel::joinable!(locations -> agents (agent_id));
diesel::joinable!(process_execution_custom_values -> process_executions (process_execution_id));
diesel::joinable!(process_execution_custom_values -> recipe_process_flow_data_fields (field_id));
diesel::joinable!(process_executions -> recipe_process_flows (process_flow_id));
diesel::joinable!(recipe_flow_template_data_fields -> recipe_flow_template_group_data_fields (group_id));
diesel::joinable!(recipe_flow_template_data_fields -> recipe_flow_templates (recipe_flow_template_id));
diesel::joinable!(recipe_flow_templates -> recipe_templates (recipe_template_id));
diesel::joinable!(recipe_process_flow_data_fields -> recipe_flow_template_data_fields (recipe_flow_template_data_field_id));
diesel::joinable!(recipe_process_flow_data_fields -> recipe_process_flow_group_data_fields (group_id));
diesel::joinable!(recipe_process_flow_data_fields -> recipe_process_flows (recipe_process_flow_id));
diesel::joinable!(recipe_process_flows -> recipe_flow_templates (recipe_flow_template_id));
diesel::joinable!(recipe_process_flows -> recipe_processes (recipe_process_id));
diesel::joinable!(recipe_processes -> recipe_templates (recipe_template_id));
diesel::joinable!(recipe_processes -> recipes (recipe_id));
diesel::joinable!(recipe_resources -> recipes (recipe_id));
diesel::joinable!(recipe_resources -> resource_specifications (resource_specification_id));
diesel::joinable!(recipe_template_blacklists -> map_templates (map_template_id));
diesel::joinable!(recipe_templates -> map_templates (map_template_id));
diesel::joinable!(recipe_templates_access -> agents (agent_id));
diesel::joinable!(recipe_templates_access -> recipe_templates (recipe_template_id));
diesel::joinable!(recipes -> agents (agent_id));
diesel::joinable!(resource_specifications -> agents (agent_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    counters,
    economic_resources,
    locations,
    map_templates,
    process_execution_custom_values,
    process_executions,
    recipe_flow_template_data_fields,
    recipe_flow_template_group_data_fields,
    recipe_flow_templates,
    recipe_process_flow_data_fields,
    recipe_process_flow_group_data_fields,
    recipe_process_flows,
    recipe_process_relations,
    recipe_processes,
    recipe_resources,
    recipe_template_blacklists,
    recipe_templates,
    recipe_templates_access,
    recipes,
    resource_specifications,
);
