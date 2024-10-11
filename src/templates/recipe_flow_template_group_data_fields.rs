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

use crate::db::schema::recipe_flow_template_group_data_fields;

use crate::db::schema::sql_types::FieldGroupClassEnum;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = FieldGroupClassEnum)]
pub enum FieldGroupClass {
    ResourceSpecification,
    EconomicResource,
    Location,
    Custom,
    ReferenceDocument
}

impl ToSql<FieldGroupClassEnum, Pg> for FieldGroupClass {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            FieldGroupClass::ResourceSpecification => out.write_all(b"ResourceSpecification")?,
            FieldGroupClass::EconomicResource => out.write_all(b"EconomicResource")?,
            FieldGroupClass::Location => out.write_all(b"Location")?,
            FieldGroupClass::Custom => out.write_all(b"Custom")?,
            FieldGroupClass::ReferenceDocument => out.write_all(b"ReferenceDocument")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FieldGroupClassEnum, Pg> for FieldGroupClass {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"ResourceSpecification" => Ok(FieldGroupClass::ResourceSpecification),
            b"EconomicResource" => Ok(FieldGroupClass::EconomicResource),
            b"Location" => Ok(FieldGroupClass::Location),
            b"Custom" => Ok(FieldGroupClass::Custom),
            b"ReferenceDocument" => Ok(FieldGroupClass::ReferenceDocument),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_flow_template_group_data_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeFlowTemplateGroupDataField {
    pub id: Uuid,
    pub name: String,
    pub group_class: FieldGroupClass
}

#[derive(Insertable)]
#[diesel(table_name = recipe_flow_template_group_data_fields)]
pub struct NewRecipeFlowTemplateGroupDataField<'a> {
    pub name: &'a str,
    pub group_class: &'a FieldGroupClass,
}

impl<'a> NewRecipeFlowTemplateGroupDataField<'a> {
    pub fn new(
        name: &'a str,
        group_class: &'a FieldGroupClass
    ) -> Self {
        NewRecipeFlowTemplateGroupDataField {
            name,
            group_class
        }
    }
}