use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow}, 
    expression::AsExpression, 
    pg::{Pg, PgValue}, 
    serialize::{self, IsNull, Output, ToSql}, 
    Insertable, 
    Queryable
};
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::db::schema::recipe_flow_templates;

use crate::db::schema::sql_types::EventTypeEnum;
use crate::db::schema::sql_types::RoleTypeEnum;


#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = EventTypeEnum)]
pub enum EventType {
    EconomicEvent
}

impl ToSql<EventTypeEnum, Pg> for EventType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            EventType::EconomicEvent => out.write_all(b"EconomicEvent")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<EventTypeEnum, Pg> for EventType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"EconomicEvent" => Ok(EventType::EconomicEvent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = RoleTypeEnum)]
pub enum RoleType {
    Input,
    Output
}

impl ToSql<RoleTypeEnum, Pg> for RoleType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RoleType::Input => out.write_all(b"Input")?,
            RoleType::Output => out.write_all(b"Output")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<RoleTypeEnum, Pg> for RoleType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Input" => Ok(RoleType::Input),
            b"Output" => Ok(RoleType::Output),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}



#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_flow_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeFlowTemplate {
    pub id: Uuid,
    pub recipe_template_id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType
}

#[derive(Insertable)]
#[diesel(table_name = recipe_flow_templates)]
pub struct NewRecipeTemplate<'a> {
    pub recipe_template_id: &'a Uuid,
    pub event_type: &'a EventType,
    pub role_type: &'a RoleType,
}

impl<'a> NewRecipeTemplate<'a> {
    pub fn new(
        recipe_template_id: &'a Uuid,
        event_type: &'a EventType,
        role_type: &'a RoleType,
    ) -> Self {
        NewRecipeTemplate {
            recipe_template_id,
            event_type,
            role_type
        }
    }
}