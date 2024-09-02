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

use crate::db::schema::{recipe_flow_templates, sql_types::ActionTypeEnum};

use crate::db::schema::sql_types::EventTypeEnum;
use crate::db::schema::sql_types::RoleTypeEnum;

use super::recipe_flow_template_data_field::{RecipeFlowTemplateDataField, RecipeFlowTemplateDataFieldInput};


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


#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone, Copy)]
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



#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone, Copy)]
#[diesel(sql_type = ActionTypeEnum)]
pub enum ActionType {
    Cite,
    Modify,
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
            ActionType::Modify => out.write_all(b"Modify")?,
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
            b"Modify" => Ok(ActionType::Modify),
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




#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_flow_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeFlowTemplate {
    pub id: Uuid,
    pub recipe_template_id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType,
    pub action: ActionType
}

#[derive(Insertable)]
#[diesel(table_name = recipe_flow_templates)]
pub struct NewRecipeFlowTemplate<'a> {
    pub recipe_template_id: &'a Uuid,
    pub event_type: &'a EventType,
    pub role_type: &'a RoleType,
    pub action: &'a ActionType,
}

impl<'a> NewRecipeFlowTemplate<'a> {
    pub fn new(
        recipe_template_id: &'a Uuid,
        event_type: &'a EventType,
        role_type: &'a RoleType,
        action: &'a ActionType,
    ) -> Self {
        NewRecipeFlowTemplate {
            recipe_template_id,
            event_type,
            role_type,
            action
        }
    }
}


#[derive(juniper::GraphQLObject)]
pub struct RecipeFlowTemplateWithDataFields {
    pub id: Uuid,
    pub recipe_template_id: Uuid,
    pub event_type: EventType,
    pub role_type: RoleType,
    pub action: ActionType,
    pub data_fields: Vec<RecipeFlowTemplateDataFieldInput>
}

impl RecipeFlowTemplateWithDataFields {
    pub fn new(recipe_flow_template: &RecipeFlowTemplate) -> Self {
        RecipeFlowTemplateWithDataFields {
            id: recipe_flow_template.id,
            recipe_template_id: recipe_flow_template.recipe_template_id,
            event_type: recipe_flow_template.event_type.clone(),
            role_type: recipe_flow_template.role_type,
            action: recipe_flow_template.action,
            data_fields: Vec::new()
        }
    }

    pub fn add_data_field(&mut self, data_field: RecipeFlowTemplateDataFieldInput) {
        self.data_fields.push(data_field);
    }
}