use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
    Insertable, Queryable,
};
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::db::schema::{
    recipe_flow_template_data_fields, 
    sql_types::{FieldClassEnum, FieldTypeEnum, FlowThroughEnum}
};

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = FieldClassEnum)]
pub enum FieldClass {
    ResourceSpecification,
    EconomicResource,
    Quantity,
    HasPointInTime,
    Agent,
    Location,
    Note,
    TrackingIdentifier,
    Custom
}

impl ToSql<FieldClassEnum, Pg> for FieldClass {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            FieldClass::ResourceSpecification => out.write_all(b"resourceSpecification")?,
            FieldClass::EconomicResource => out.write_all(b"economicResource")?,
            FieldClass::Quantity => out.write_all(b"quantity")?,
            FieldClass::HasPointInTime => out.write_all(b"hasPointInTime")?,
            FieldClass::Agent => out.write_all(b"agent")?,
            FieldClass::Location => out.write_all(b"location")?,
            FieldClass::TrackingIdentifier => out.write_all(b"trackingIdentifier")?,
            FieldClass::Note => out.write_all(b"note")?,
            FieldClass::Custom => out.write_all(b"custom")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FieldClassEnum, Pg> for FieldClass {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"resourceSpecification" => Ok(FieldClass::ResourceSpecification),
            b"economicResource" => Ok(FieldClass::EconomicResource),
            b"quantity" => Ok(FieldClass::Quantity),
            b"hasPointInTime" => Ok(FieldClass::HasPointInTime),
            b"agent" => Ok(FieldClass::Agent),
            b"location" => Ok(FieldClass::Location),
            b"trackingIdentifier" => Ok(FieldClass::TrackingIdentifier),
            b"note" => Ok(FieldClass::Note),
            b"custom" => Ok(FieldClass::Custom),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = FieldTypeEnum)]
pub enum FieldType {
    Text,
    Date,
    Number,
    Select,
}

impl ToSql<FieldTypeEnum, Pg> for FieldType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            FieldType::Text => out.write_all(b"Text")?,
            FieldType::Date => out.write_all(b"Date")?,
            FieldType::Number => out.write_all(b"Number")?,
            FieldType::Select => out.write_all(b"Select")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FieldTypeEnum, Pg> for FieldType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Text" => Ok(FieldType::Text),
            b"Date" => Ok(FieldType::Date),
            b"Number" => Ok(FieldType::Number),
            b"Select" => Ok(FieldType::Select),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = FlowThroughEnum)]
pub enum FlowThrough {
    Internal,
    External,
}

// ToSql implementation
impl ToSql<FlowThroughEnum, Pg> for FlowThrough {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            FlowThrough::Internal => out.write_all(b"Internal")?,
            FlowThrough::External => out.write_all(b"External")?,
        }
        Ok(IsNull::No)
    }
}

// FromSql implementation
impl FromSql<FlowThroughEnum, Pg> for FlowThrough {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Internal" => Ok(FlowThrough::Internal),
            b"External" => Ok(FlowThrough::External),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_flow_template_data_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeFlowTemplateDataField {
    pub id: Uuid,
    pub recipe_flow_template_id: Uuid,
    pub group_id: Option<Uuid>,
    pub field_identifier: String,
    pub field_class: FieldClass,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub flow_through: Option<FlowThrough>,
    pub inherits: Option<Uuid>
}



#[derive(Insertable)]
#[diesel(table_name = recipe_flow_template_data_fields)]
pub struct NewRecipeFlowTemplateDataField<'a> {
    pub recipe_flow_template_id: &'a Uuid,
    pub group_id: Option<&'a Uuid>,
    pub field_identifier: &'a str,
    pub field_class: &'a FieldClass,
    pub field: &'a str,
    pub field_type: &'a FieldType,
    pub note: Option<&'a str>,
    pub required: &'a bool,
    pub flow_through: Option<&'a FlowThrough>,
    pub inherits: Option<&'a Uuid>
}

impl<'a> NewRecipeFlowTemplateDataField<'a> {
    pub fn new(
        recipe_flow_template_id: &'a Uuid,
        group_id: Option<&'a Uuid>,
        field_identifier: &'a str,
        field_class: &'a FieldClass,
        field: &'a str,
        field_type: &'a FieldType,
        note: Option<&'a str>,
        required: &'a bool,
        flow_through: Option<&'a FlowThrough>,
        inherits: Option<&'a Uuid>
    ) -> Self {
        NewRecipeFlowTemplateDataField {
            recipe_flow_template_id,
            group_id,
            field_identifier,
            field_class,
            field,
            field_type,
            note,
            required,
            flow_through,
            inherits
        }
    }
}

#[derive(juniper::GraphQLObject, Debug, Clone)]
pub struct RecipeFlowTemplateDataFieldInput {
    pub id: Uuid,
    pub recipe_flow_template_id: Uuid,
    pub group_id: Option<Uuid>,
    pub field_identifier: String,
    pub field_class: FieldClass,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub flow_through: Option<FlowThrough>,
    pub inherits: Option<Uuid>
}


impl TryFrom<&RecipeFlowTemplateDataField> for RecipeFlowTemplateDataFieldInput {
    type Error = String; // Define your error type here

    fn try_from(value: &RecipeFlowTemplateDataField) -> Result<Self, Self::Error> {
        Ok(RecipeFlowTemplateDataFieldInput {
            id: value.id,
            recipe_flow_template_id: value.recipe_flow_template_id,
            group_id: value.group_id,
            field_identifier: value.field_identifier.clone(),
            field_class: value.field_class.clone(),
            field: value.field.clone(),
            field_type: value.field_type.clone(),
            note: value.note.clone(),
            required: value.required,
            flow_through: value.flow_through.clone(),
            inherits: value.inherits
        })
    }
}

