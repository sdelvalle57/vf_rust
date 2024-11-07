use diesel::prelude::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::{recipe_process_relations, recipe_processes};

#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = recipe_processes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeProcess {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub recipe_template_id: Uuid,
    pub name: String,
    pub node_id: String
}

#[derive(Insertable)]
#[diesel(table_name = recipe_processes)]
pub struct NewRecipeProcess<'a> {
    pub recipe_id: &'a Uuid,
    pub recipe_template_id: &'a Uuid,
    pub name: &'a str,
    pub node_id: &'a str
}

impl<'a>  NewRecipeProcess<'a> {
    pub fn new(
        recipe_id: &'a Uuid,
        recipe_template_id: &'a Uuid,
        name: &'a str,
        node_id: &'a str
    ) -> Self {
        NewRecipeProcess {
            recipe_id,
            recipe_template_id, 
            name,
            node_id
        }
    }
}


#[derive(Queryable, GraphQLObject, Debug, Clone)]
#[diesel(table_name = recipe_process_relations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProcessRelation {
    pub id: Uuid,
    pub recipe_process_id: Uuid,
    pub predecessor: Uuid,
    pub recipe_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = recipe_process_relations)]
pub struct NewProcessRelation<'a> {
    pub recipe_id: &'a Uuid,
    pub recipe_process_id: &'a Uuid,
    pub predecessor: &'a Uuid
}

impl<'a> NewProcessRelation<'a> {
    pub fn new(
        recipe_id: &'a Uuid,
        recipe_process_id: &'a Uuid,
        predecessor: &'a Uuid,
    ) -> Self {
        NewProcessRelation {
            recipe_id,
            recipe_process_id,
            predecessor
        }
    }
}


#[derive(GraphQLObject, Debug)]
pub struct RecipeProcessResponse {
    pub recipe_process: RecipeProcess,
    pub predecessors: Vec<Uuid>,
}

impl RecipeProcessResponse {
    pub fn new(recipe_process: RecipeProcess) -> Self {
        RecipeProcessResponse {
            recipe_process,
            predecessors: Vec::new()
        }
    }

    pub fn add_predecessor(&mut self, id: Uuid) {
        self.predecessors.push(id)
    }
}

