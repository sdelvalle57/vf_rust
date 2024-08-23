use std::io::Write;

use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow}, 
    expression::AsExpression, 
    pg::{Pg, PgValue}, 
    serialize::{self, IsNull, Output, ToSql}, 
    sql_types::SqlType, 
    Insertable, 
    Queryable
};
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::db::schema::{resource_specifications, sql_types::ResourceTypeEnum};

#[derive(SqlType)]
#[diesel(postgres_type(name = "ResourceTypeEnum"))]
pub struct ResourceTypeStruct;


#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum)]
#[diesel(sql_type = ResourceTypeEnum)]
pub enum ResourceType {
    Resource,
    Product,
    Asset,
}

impl ToSql<ResourceTypeEnum, Pg> for ResourceType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ResourceType::Resource => out.write_all(b"Resource")?,
            ResourceType::Product => out.write_all(b"Product")?,
            ResourceType::Asset => out.write_all(b"Asset")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ResourceTypeEnum, Pg> for ResourceType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Resource" => Ok(ResourceType::Resource),
            b"Product" => Ok(ResourceType::Product),
            b"Asset" => Ok(ResourceType::Asset),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Queryable, Debug, GraphQLObject)]
#[diesel(table_name = resource_specifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResourceSpecification {
    pub id: Uuid,
    pub agent_id: Uuid,  
    pub name: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub resource_type: ResourceType,  
    pub unit_of_measure: String
}

// Updated Insertable struct
#[derive(Insertable, Debug)]
#[diesel(table_name = resource_specifications)]
pub struct NewResourceSpecification<'a> {
    pub agent_id: &'a Uuid,
    pub name: &'a str,
    pub note: Option<&'a str>,
    pub resource_type: &'a ResourceType,
    pub unit_of_measure: &'a str
}

impl<'a> NewResourceSpecification<'a> {
    pub fn new(
        agent_id: &'a Uuid,
        name: &'a str,
        note: Option<&'a str>,
        resource_type: &'a ResourceType,
        unit_of_measure: &'a str
    ) -> Self {
        NewResourceSpecification {
            agent_id,
            name,
            note,
            resource_type,
            unit_of_measure
        }
    }
}
