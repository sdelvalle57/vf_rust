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

use super::recipe_flow_template::{ActionType, RecipeFlowTemplateWithDataFields};


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
    pub identifier: String,
    pub name: String,
    pub commitment: Option<ActionType>,
    pub fulfills: Option<Uuid>,
    pub recipe_template_type: RecipeTemplateType
}

#[derive(Insertable)]
#[diesel(table_name = recipe_templates)]
pub struct NewRecipeTemplate<'a> {
    pub identifier: &'a str,
    pub name: &'a str,
    pub commitment: Option<&'a ActionType>,
    pub fulfills: Option<&'a Uuid>,
    pub recipe_template_type: &'a RecipeTemplateType,
}

impl<'a> NewRecipeTemplate<'a> {
    pub fn new(
        identifier: &'a str,
        name: &'a str,
        commitment: Option<&'a ActionType>,
        fulfills: Option<&'a Uuid>,
        recipe_template_type: &'a RecipeTemplateType
    ) -> Self {
        NewRecipeTemplate {
            identifier,
            name,
            commitment,
            fulfills,
            recipe_template_type
        }
    }
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct RecipeTemplateWithRecipeFlows {
    pub id: Uuid,
    pub name: String,
    pub recipe_template_type: RecipeTemplateType,
    pub commitment: Option<ActionType>,
    pub fulfills: Option<Uuid>,
    pub recipe_flows: Vec<RecipeFlowTemplateWithDataFields>
}

impl RecipeTemplateWithRecipeFlows {
    pub fn new(recipe_template: & RecipeTemplate) -> Self {
        RecipeTemplateWithRecipeFlows {
            id: recipe_template.id,
            name: recipe_template.name.clone(),
            recipe_template_type: recipe_template.recipe_template_type.clone(),
            commitment: recipe_template.commitment,
            fulfills: recipe_template.fulfills,
            recipe_flows: Vec::new()
        }
    }

    pub fn add_recipe_flow(&mut self, recipe_flow: RecipeFlowTemplateWithDataFields) {
        self.recipe_flows.push(recipe_flow)
    }

}