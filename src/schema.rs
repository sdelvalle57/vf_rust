// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "action_type_enum"))]
    pub struct ActionTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resource_type_enum"))]
    pub struct ResourceTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_enum"))]
    pub struct RoleEnum;
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

    economic_events (id) {
        id -> Uuid,
        recipe_event_id -> Nullable<Uuid>,
        provider_id -> Nullable<Uuid>,
        receiver_id -> Nullable<Uuid>,
        note -> Nullable<Text>,
        resource_specification_id -> Nullable<Uuid>,
        resource_inventoried_as -> Nullable<Uuid>,
        resource_quantity -> Numeric,
        to_resource_specification_id -> Nullable<Uuid>,
        to_unit_of_measure -> Nullable<Text>,
        has_point_in_time -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    economic_resources (id) {
        id -> Uuid,
        resource_specification_id -> Uuid,
        name -> Text,
        note -> Nullable<Text>,
        accounting_quantity -> Numeric,
        on_hand_quantity -> Numeric,
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

    processes (id) {
        id -> Uuid,
        recipe_id -> Nullable<Uuid>,
        name -> Text,
        note -> Nullable<Text>,
        output_of -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActionTypeEnum;
    use super::sql_types::RoleEnum;

    recipe_events (id) {
        id -> Uuid,
        process_id -> Nullable<Uuid>,
        action -> ActionTypeEnum,
        role -> RoleEnum,
        resource_specification_id -> Nullable<Uuid>,
        economic_resource_id -> Nullable<Uuid>,
        note -> Nullable<Text>,
        is_commitment -> Nullable<Bool>,
        commitment_status -> Nullable<Text>,
        triggers_commitment -> Nullable<Bool>,
        fulfills_commitment_id -> Nullable<Uuid>,
        location -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    recipe_resources (id) {
        id -> Uuid,
        recipe_id -> Nullable<Uuid>,
        resource_specification_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    recipes (id) {
        id -> Uuid,
        agent_id -> Nullable<Uuid>,
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

diesel::joinable!(economic_events -> economic_resources (resource_inventoried_as));
diesel::joinable!(economic_events -> recipe_events (recipe_event_id));
diesel::joinable!(economic_resources -> resource_specifications (resource_specification_id));
diesel::joinable!(processes -> recipes (recipe_id));
diesel::joinable!(recipe_events -> economic_resources (economic_resource_id));
diesel::joinable!(recipe_events -> processes (process_id));
diesel::joinable!(recipe_events -> resource_specifications (resource_specification_id));
diesel::joinable!(recipe_resources -> recipes (recipe_id));
diesel::joinable!(recipe_resources -> resource_specifications (resource_specification_id));
diesel::joinable!(recipes -> agents (agent_id));
diesel::joinable!(resource_specifications -> agents (agent_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    economic_events,
    economic_resources,
    processes,
    recipe_events,
    recipe_resources,
    recipes,
    resource_specifications,
);
