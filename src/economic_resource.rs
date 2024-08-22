
use chrono::NaiveDateTime;
use diesel::Queryable;
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::economic_resources;


// #[derive(Queryable, GraphQLObject, Debug)]
// #[diesel(table_name = economic_resources)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct EconomicResource {
//     pub id: Uuid,
//     pub resource_specification_id: Uuid,
//     pub name: String,
//     pub note: Option<String>,
//     pub accounting_quantity: f64,
//     pub on_hand_quantity: f64,
//     pub unit_of_measure: String,
//     pub tracking_identifier: Option<String>,
//     pub current_location: String,
//     pub lot: Option<String>,
//     pub contained_in: Option<Uuid>,
//     pub created_at: NaiveDateTime,
//     pub reference_number: i32,
// }

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