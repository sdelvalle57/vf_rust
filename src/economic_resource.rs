use std::io::Write;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, AsExpression, FromSqlRow};
use diesel::sql_types::Text;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use diesel::pg::{Pg, PgValue};
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::db::schema::economic_resources;

#[derive(Debug, Clone, Copy, GraphQLEnum, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
#[graphql(description = "The type of the resource")]
pub enum ResourceType {
    PRODUCT,
    RESOURCE,
    ASSET,
}

// Implement how to serialize the ResourceType to the database
impl ToSql<Text, Pg> for ResourceType {
    fn to_sql<'b>(&self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = match *self {
            ResourceType::PRODUCT => "PRODUCT",
            ResourceType::RESOURCE => "RESOURCE",
            ResourceType::ASSET => "ASSET",
        };
        out.write_all(value.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

// Implement how to deserialize the ResourceType from the database
impl FromSql<Text, Pg> for ResourceType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"PRODUCT" => Ok(ResourceType::PRODUCT),
            b"RESOURCE" => Ok(ResourceType::RESOURCE),
            b"ASSET" => Ok(ResourceType::ASSET),
            _ => Err("Unrecognized resource type".into()),
        }
    }
}

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = economic_resources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EconomicResource {
    pub id: Uuid,
    pub resource_specification_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub accounting_quantity: f64,
    pub on_hand_quantity: f64,
    pub unit_of_measure: String,
    pub tracking_identifier: Option<String>,
    pub current_location: String,
    pub lot: Option<String>,
    pub contained_in: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub resource_type: ResourceType,
    pub reference_number: i32,
}

// #[derive(Insertable)]
// #[diesel(table_name = resource_specifications)]
// pub struct NewEconomicResource<'a> {
//     pub resource_specification_id: &'a Uuid,
//     pub name: &'a str,
//     pub note: Option<&'a str>,
//     pub accounting_quantity: f64,
//     pub unit_of_measure: &'a str,
//     pub tracking_identifier: Option<String>,
//     pub current_location: String,
//     pub lot: Option<String>,
//     pub contained_in: Option<Uuid>,
// }

// impl<'a> NewEconomicResource<'a> {
//     pub fn new(
//         agent_id: &'a Uuid,
//         name: &'a str,
//         note: Option<&'a str>,
//     ) -> Self {
//         NewEconomicResource {
//             agent_id,
//             name,
//             note,
//         }
//     }
// }