use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::db::schema::processes;

use super::recipe::RecipeWithResources;

#[derive(Queryable, GraphQLObject, Debug)]
#[diesel(table_name = recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Process {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub output_of: Option<Uuid>,
    pub template_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = processes)]
pub struct NewProcess<'a> {
    pub recipe_id: &'a Uuid,
    pub name: &'a str,
    pub note: Option<&'a str>,
    pub output_of: Option<&'a Uuid>,
    pub template_id: &'a Uuid
}

impl<'a> NewProcess<'a> {
    pub fn new(
        recipe_id: &'a Uuid,
        name: &'a str,
        note: Option<&'a str>,
        output_of: Option<&'a Uuid>,
        template_id:  &'a Uuid
    ) -> Self {
        NewProcess {
            recipe_id,
            name,
            note,
            output_of,
            template_id
        }
    }
}


#[derive(GraphQLObject, Debug)]
pub struct ProcessWithRecipe {
    process: Process,
    recipe: RecipeWithResources
}

impl ProcessWithRecipe {
    pub fn new(process: Process, recipe: RecipeWithResources) -> Self {
        ProcessWithRecipe {
            process,
            recipe
        }
    }
}