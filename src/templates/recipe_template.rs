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

use crate::db::schema::recipe_templates;

use crate::db::schema::sql_types::RecipeTemplateTypeEnum;

use super::recipe_flow_template::RecipeFlowTemplateWithDataFields;


#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = RecipeTemplateTypeEnum)]
pub enum RecipeTemplateType {
    FDA,
    Custom
}

impl ToSql<RecipeTemplateTypeEnum, Pg> for RecipeTemplateType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RecipeTemplateType::Custom => out.write_all(b"Custom")?,
            RecipeTemplateType::FDA => out.write_all(b"FDA")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<RecipeTemplateTypeEnum, Pg> for RecipeTemplateType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Custom" => Ok(RecipeTemplateType::Custom),
            b"FDA" => Ok(RecipeTemplateType::FDA),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipe_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RecipeTemplate {
    pub id: Uuid,
    pub name: String,
    pub recipe_template_type: RecipeTemplateType
}

#[derive(Insertable)]
#[diesel(table_name = recipe_templates)]
pub struct NewRecipeTemplate<'a> {
    pub name: &'a str,
    pub recipe_template_type: &'a RecipeTemplateType,
}

impl<'a> NewRecipeTemplate<'a> {
    pub fn new(
        name: &'a str,
        recipe_template_type: &'a RecipeTemplateType
    ) -> Self {
        NewRecipeTemplate {
            name,
            recipe_template_type
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct RecipeTemplateWithRecipeFlows {
    pub id: Uuid,
    pub name: String,
    pub recipe_template_type: RecipeTemplateType,
    pub recipe_flows: Vec<RecipeFlowTemplateWithDataFields>
}

impl RecipeTemplateWithRecipeFlows {
    pub fn new(recipe_template: & RecipeTemplate) -> Self {
        RecipeTemplateWithRecipeFlows {
            id: recipe_template.id,
            name: recipe_template.name.clone(),
            recipe_template_type: recipe_template.recipe_template_type.clone(),
            recipe_flows: Vec::new()
        }
    }

    pub fn add_recipe_flow(&mut self, recipe_flow: RecipeFlowTemplateWithDataFields) {
        self.recipe_flows.push(recipe_flow)
    }

}