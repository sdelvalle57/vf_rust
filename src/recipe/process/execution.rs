

use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::{db::schema::process_executions, templates::recipe_flow_template::{ActionType, RoleType}};

#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = process_executions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProcessExecution {
    pub id: Uuid,
    pub process_flow_id: Uuid,
    pub action: ActionType,
    pub role_type: RoleType,
    pub resource_specification: Option<Uuid>,
    pub resource_reference_number: Option<i32>,
    pub resource_lot_number: Option<i32>,
    pub resource_quantity: Option<i32>,
    pub to_resource_specification: Option<Uuid>,
    pub to_resource_reference_number: Option<i32>,
    pub to_resource_lot_number: Option<i32>,
    pub provider_agent: Uuid,
    pub receiver_agent: Uuid,
    pub at_location: Option<Uuid>,
    pub to_location: Option<Uuid>,
    pub has_point_in_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub corrects: Option<Uuid>,
    pub note: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = process_executions)]
pub struct NewProcessExecution<'a> {
    pub process_flow_id: &'a Uuid,
    pub action: &'a ActionType,
    pub role_type: &'a RoleType,
    pub resource_specification: Option<&'a Uuid>,
    pub resource_reference_number: Option<&'a i32>,
    pub resource_lot_number: Option<&'a i32>,
    pub resource_quantity: Option<&'a i32>,
    pub to_resource_specification: Option<&'a Uuid>,
    pub to_resource_reference_number: Option<&'a i32>,
    pub to_resource_lot_number: Option<&'a i32>,
    pub provider_agent:&'a Uuid,
    pub receiver_agent:&'a Uuid,
    pub at_location: Option<&'a Uuid>,
    pub to_location: Option<&'a Uuid>,
    pub has_point_in_time: Option<&'a NaiveDateTime>,
    pub corrects: Option<&'a Uuid>,
    pub note: Option<&'a String>
}

impl<'a>  NewProcessExecution<'a> {
    pub fn new(
        process_flow_id: &'a Uuid,
        action: &'a ActionType,
        role_type: &'a RoleType,
        resource_specification: Option<&'a Uuid>,
        resource_reference_number: Option<&'a i32>,
        resource_lot_number: Option<&'a i32>,
        resource_quantity: Option<&'a i32>,
        to_resource_specification: Option<&'a Uuid>,
        to_resource_reference_number: Option<&'a i32>,
        to_resource_lot_number: Option<&'a i32>,
        provider_agent:&'a Uuid,
        receiver_agent:&'a Uuid,
        at_location: Option<&'a Uuid>,
        to_location: Option<&'a Uuid>,
        has_point_in_time: Option<&'a NaiveDateTime>,
        corrects: Option<&'a Uuid>,
        note: Option<&'a String>
    ) -> Self {
        NewProcessExecution {
            process_flow_id,
            action,
            role_type,
            resource_specification,
            resource_reference_number,
            resource_lot_number,
            resource_quantity,
            to_resource_specification,
            to_resource_reference_number,
            to_resource_lot_number,
            provider_agent,
            receiver_agent,
            at_location,
            to_location,
            has_point_in_time,
            corrects,
            note
        }
    }
}
