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

use crate::db::schema::{map_templates, sql_types::TemplateTypeEnum};

use super::{recipe_template::RecipeTemplateWithRecipeFlows, recipe_template_blacklist::RecipeTemplateBlacklist};

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = TemplateTypeEnum)]
pub enum TemplateType {
    FDA,
    Custom
}

impl ToSql<TemplateTypeEnum, Pg> for TemplateType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TemplateType::Custom => out.write_all(b"Custom")?,
            TemplateType::FDA => out.write_all(b"FDA")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<TemplateTypeEnum, Pg> for TemplateType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Custom" => Ok(TemplateType::Custom),
            b"FDA" => Ok(TemplateType::FDA),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = map_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct MapTemplate {
    pub id: Uuid,
    pub name: String,
    pub type_: TemplateType
}

#[derive(Insertable)]
#[diesel(table_name = map_templates)]
pub struct NewMapTemplate<'a> {
    pub name: &'a str,
    pub type_: &'a TemplateType
}

impl<'a> NewMapTemplate<'a> {
    pub fn new(
        name: &'a str,
        type_: &'a TemplateType
    ) -> Self {
        NewMapTemplate {
            name,
            type_
        }
    }
}

#[derive(GraphQLObject)]
pub struct MapTemplateResponse {
    pub map: MapTemplate,
    pub templates: Vec<RecipeTemplateWithRecipeFlows>,
    pub blacklists: Vec<RecipeTemplateBlacklist>
}

impl MapTemplateResponse {
    pub fn new(map: MapTemplate, blacklists: Vec<RecipeTemplateBlacklist>) -> Self {
        MapTemplateResponse {
            map,
            templates: Vec::new(),
            blacklists
        }
    }

    pub fn add_template(&mut self, template: RecipeTemplateWithRecipeFlows) {
        self.templates.push(template)
    }
}