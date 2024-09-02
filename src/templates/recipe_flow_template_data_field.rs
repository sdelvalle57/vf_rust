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

use crate::db::schema::recipe_flow_template_data_fields;

use crate::db::schema::sql_types::FieldTypeEnum;
use crate::db::schema::sql_types::FieldValueEnum;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = FieldValueEnum)]
pub enum FieldValue {
    Product,
    Quantity,
    HasPointInTime,
    AtLocation,
    Note
}

impl ToSql<FieldValueEnum, Pg> for FieldValue {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            FieldValue::Product => out.write_all(b"product")?,
            FieldValue::Quantity => out.write_all(b"quantity")?,
            FieldValue::HasPointInTime => out.write_all(b"hasPointInTime")?,
            FieldValue::AtLocation => out.write_all(b"atLocation")?,
            FieldValue::Note => out.write_all(b"note")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FieldValueEnum, Pg> for FieldValue {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"product" => Ok(FieldValue::Product),
            b"quantity" => Ok(FieldValue::Quantity),
            b"hasPointInTime" => Ok(FieldValue::HasPointInTime),
            b"atLocation" => Ok(FieldValue::AtLocation),
            b"note" => Ok(FieldValue::Note),
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

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_flow_template_data_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeFlowTemplateDataField {
    pub id: Uuid,
    pub recipe_flow_template_id: Uuid,
    pub field_value: FieldValue,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub query: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = recipe_flow_template_data_fields)]
pub struct NewRecipeFlowTemplateDataField<'a> {
    pub recipe_flow_template_id: &'a Uuid,
    pub field_value: &'a FieldValue,
    pub field: &'a str,
    pub field_type: &'a FieldType,
    pub note: Option<&'a str>,
    pub required: &'a bool,
    pub query: Option<&'a Uuid>,
    pub default_value: Option<&'a str>,
}

impl<'a> NewRecipeFlowTemplateDataField<'a> {
    pub fn new(
        recipe_flow_template_id: &'a Uuid,
        field_value: &'a FieldValue,
        field: &'a str,
        field_type: &'a FieldType,
        note: Option<&'a str>,
        required: &'a bool,
        query: Option<&'a Uuid>,
        default_value: Option<&'a str>,
    ) -> Self {
        NewRecipeFlowTemplateDataField {
            recipe_flow_template_id,
            field_value,
            field,
            field_type,
            note,
            required,
            query,
            default_value,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct RecipeFlowTemplateDataFieldInput {
    pub field_value: FieldValue,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub default_value: Option<String>,
}

impl TryFrom<RecipeFlowTemplateDataField> for RecipeFlowTemplateDataFieldInput {
    type Error = String; // Define your error type here

    fn try_from(value: RecipeFlowTemplateDataField) -> Result<Self, Self::Error> {
        Ok(RecipeFlowTemplateDataFieldInput {
            field_value: value.field_value,
            field: value.field,
            field_type: value.field_type,
            note: value.note,
            required: value.required,
            default_value: value.default_value,
        })
    }
}

