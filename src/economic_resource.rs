
use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::{db::schema::economic_resources::{self}, resource_specification::ResourceSpecification};

// Wrapper around BigDecimal



#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = economic_resources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EconomicResource {
    pub id: Uuid,
    pub resource_specification_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub accounting_quantity: i32,
    pub on_hand_quantity: i32,
    pub tracking_identifier: Option<String>,
    pub current_location: String,
    pub lot: Option<String>,
    pub contained_in: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub reference_number: i32,
}

#[derive(Insertable)]
#[diesel(table_name = economic_resources)]
pub struct NewEconomicResource<'a> {
    pub resource_specification_id: &'a Uuid,
    pub name: &'a str,
    pub note: Option<&'a str>,
    pub accounting_quantity: &'a i32,
    pub on_hand_quantity: &'a i32,
    pub tracking_identifier: Option<&'a str>,
    pub current_location: &'a str,
    pub lot: Option<&'a str>,
    pub contained_in: Option<&'a Uuid>,
}

impl<'a> NewEconomicResource<'a> {
    pub fn new(
        resource_specification_id: &'a Uuid,
        name: &'a str,
        note: Option<&'a str>,
        accounting_quantity: &'a i32,
        tracking_identifier: Option<&'a str>,
        current_location: &'a str,
        lot: Option<&'a str>,
        contained_in: Option<&'a Uuid>

    ) -> Self {
        NewEconomicResource {
            resource_specification_id,
            name,
            note,
            accounting_quantity,
            on_hand_quantity: accounting_quantity,
            tracking_identifier, 
            current_location,
            lot,
            contained_in
        }
    }
}



#[derive(GraphQLObject, Debug)]
pub struct EconomicResourceWithSpec {
    pub id: Uuid,
    pub resource_specification: ResourceSpecification,
    pub name: String,
    pub note: Option<String>,
    pub accounting_quantity: i32,
    pub on_hand_quantity: i32,
    pub tracking_identifier: Option<String>,
    pub current_location: String,
    pub lot: Option<String>,
    pub contained_in: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub reference_number: i32,
}

impl EconomicResourceWithSpec {
    pub fn build(
        economic_resource: EconomicResource,
        resource_specification: ResourceSpecification, 
    ) -> EconomicResourceWithSpec {
        EconomicResourceWithSpec {
            id: economic_resource.id,
            resource_specification,
            name: economic_resource.name,
            note: economic_resource.note,
            accounting_quantity: economic_resource.accounting_quantity,
            on_hand_quantity: economic_resource.on_hand_quantity,
            tracking_identifier: economic_resource.tracking_identifier,
            current_location: economic_resource.current_location,
            lot: economic_resource.lot,
            contained_in: economic_resource.contained_in,
            created_at: economic_resource.created_at,
            reference_number: economic_resource.reference_number
        }
    }
}