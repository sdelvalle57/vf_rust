use std::io::Write;

use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow}, 
    expression::AsExpression, 
    pg::{Pg, PgValue}, 
    serialize::{self, Output, ToSql}, 
    sql_types::{SqlType, Text}, 
    Insertable, 
    Queryable
};
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::db::schema::resource_specifications;

// #[derive(SqlType)]
// #[diesel(postgres_type(name = "resource_type_enum"))]
// pub struct ResourceTypeEnum;

// #[derive(Debug, GraphQLEnum, FromSqlRow)]
// pub enum ResourceType {
//     Product,
//     Resource,
//     Asset,
// }

// // Implement ToSql for ResourceType
// impl ToSql<ResourceTypeEnum, Pg> for ResourceType {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
//         match *self {
//             ResourceType::Product => out.write_all(b"Product")?,
//             ResourceType::Resource => out.write_all(b"Resource")?,
//             ResourceType::Asset => out.write_all(b"Asset")?,
//         }
//         Ok(serialize::IsNull::No)
//     }
// }

// // Implement FromSql for ResourceType
// impl FromSql<ResourceTypeEnum, Pg> for ResourceType {
//     fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
//         match bytes.as_bytes() {
//             b"Product" => Ok(ResourceType::Product),
//             b"Resource" => Ok(ResourceType::Resource),
//             b"Asset" => Ok(ResourceType::Asset),
//             _ => Err("Unrecognized enum variant".into()),
//         }
//     }
// }

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = resource_specifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResourceSpecification {
    pub id: Uuid,
    pub agent_id: Uuid,  
    pub name: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub resource_type: String  // Updated to use ResourceType enum
}

#[derive(Insertable)]
#[diesel(table_name = resource_specifications)]
pub struct NewResourceSpecification<'a> {
    pub agent_id: &'a Uuid,
    pub name: &'a str,
    pub note: Option<&'a str>,
    pub resource_type: &'a str  // Updated to use ResourceType enum
}

impl<'a> NewResourceSpecification<'a> {
    pub fn new(
        agent_id: &'a Uuid,
        name: &'a str,
        note: Option<&'a str>,
        resource_type: &'a str,
    ) -> Self {
        NewResourceSpecification {
            agent_id,
            name,
            note,
            resource_type
        }
    }
}
