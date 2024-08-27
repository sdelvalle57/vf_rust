use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow}, 
    expression::AsExpression, 
    pg::{Pg, PgValue}, 
    prelude::{Insertable, Queryable}, 
    serialize::{self, IsNull, Output, ToSql}
};
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::db::schema::{data_events, sql_types::{ActionTypeEnum, EventTypeEnum, RoleEnum}};

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = EventTypeEnum)]
pub enum EventType {
    EconomicEvent,
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
#[diesel(sql_type = RoleEnum)]
pub enum RoleType {
    Input,
    Output
}

impl ToSql<RoleEnum, Pg> for RoleType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RoleType::Input => out.write_all(b"Input")?,
            RoleType::Output => out.write_all(b"Output")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<RoleEnum, Pg> for RoleType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Input" => Ok(RoleType::Input),
            b"Output" => Ok(RoleType::Output),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}



#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = ActionTypeEnum)]
pub enum ActionType {
    Cite,
    Produce,
    Consume,
    Transfer,
    Use,
    Load,
    Unload
}

impl ToSql<ActionTypeEnum, Pg> for ActionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ActionType::Cite => out.write_all(b"Cite")?,
            ActionType::Produce => out.write_all(b"Produce")?,
            ActionType::Consume => out.write_all(b"Consume")?,
            ActionType::Transfer => out.write_all(b"Transfer")?,
            ActionType::Use => out.write_all(b"Use")?,
            ActionType::Load => out.write_all(b"Load")?,
            ActionType::Unload => out.write_all(b"Unload")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ActionTypeEnum, Pg> for ActionType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Cite" => Ok(ActionType::Cite),
            b"Produce" => Ok(ActionType::Produce),
            b"Consume" => Ok(ActionType::Consume),
            b"Transfer" => Ok(ActionType::Transfer),
            b"Use" => Ok(ActionType::Use),
            b"Load" => Ok(ActionType::Load),
            b"Unload" => Ok(ActionType::Unload),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Queryable, Debug, GraphQLObject, Clone)]
#[diesel(table_name = data_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DataEvent {
    pub id: Uuid,
    pub template_id: Uuid,  
    pub event_type: EventType,
    pub role: RoleType,
    pub action: ActionType
}


#[derive(Insertable, Debug)]
#[diesel(table_name = data_events)]
pub struct NewDataEvent<'a> {
    pub template_id: &'a Uuid,
    pub event_type: &'a EventType,
    pub role: &'a RoleType,
    pub action: &'a ActionType
}


impl<'a> NewDataEvent<'a> {
    pub fn new(
        template_id: &'a Uuid,
        event_type: &'a EventType,
        role: &'a RoleType,
        action: &'a ActionType
    ) -> Self {
        NewDataEvent {
            template_id,
            event_type,
            role,
            action
        }
    }
}