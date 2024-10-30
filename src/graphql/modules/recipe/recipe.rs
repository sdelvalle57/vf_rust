use crate::{
    common::resource_specification::ResourceSpecification,
    db::schema::{recipe_processes, recipe_resources, recipes},
    graphql::{
        context::Context, modules::{common::resource_specification::resource_specification_by_id, templates::template::{get_blacklists_by_template_id, BlacklistResponse}},
    },
    recipe::{
        process::process::{NewRecipeProcess, RecipeProcess},
        recipe::{NewRecipe, NewRecipeResource, Recipe, RecipeResource, RecipeWithResources},
    }, templates::recipe_template_blacklist::RecipeTemplateBlacklist,
};
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult, GraphQLInputObject};
use uuid::Uuid;

#[derive(GraphQLInputObject)]
pub struct RecipeProcessRelations {
    pub recipe_process_id: Uuid,
    pub output_of: Uuid,
}

/** Queries */
pub fn recipe_by_id(context: &Context, recipe_id: Uuid) -> FieldResult<RecipeWithResources> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let recipe = recipes::table
        .filter(recipes::id.eq(recipe_id))
        .first::<Recipe>(conn)?;

    let mut resources: Vec<ResourceSpecification> = Vec::new();

    let recipe_resources: Vec<RecipeResource> = recipe_resources::table
        .filter(recipe_resources::recipe_id.eq(recipe_id))
        .load::<RecipeResource>(conn)?;

    for resource in recipe_resources {
        let spec = resource_specification_by_id(context, resource.resource_specification_id)?;
        resources.push(spec)
    }

    Ok(RecipeWithResources::new(recipe, resources))
}

pub fn recipes_by_agent(
    context: &Context,
    agent_id: Uuid,
) -> FieldResult<Vec<RecipeWithResources>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut recipes_response: Vec<RecipeWithResources> = Vec::new();

    let recipes: Vec<Recipe> = recipes::table
        .filter(recipes::agent_id.eq(agent_id))
        .load::<Recipe>(conn)?;

    for recipe in recipes {
        let mut resources: Vec<ResourceSpecification> = Vec::new();

        let recipe_resources: Vec<RecipeResource> = recipe_resources::table
            .filter(recipe_resources::recipe_id.eq(recipe.id))
            .load::<RecipeResource>(conn)?;

        for resource in recipe_resources {
            let spec = resource_specification_by_id(context, resource.resource_specification_id)?;
            resources.push(spec)
        }

        recipes_response.push(RecipeWithResources::new(recipe, resources))
    }

    Ok(recipes_response)
}

/*** Mutations */
pub fn create_recipe(
    context: &Context,
    agent_id: Uuid,
    name: String,
    note: Option<String>,
    recipe_resources: Vec<Uuid>,
) -> FieldResult<RecipeWithResources> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut resources: Vec<ResourceSpecification> = Vec::new();
    for &resource in &recipe_resources {
        let spec = resource_specification_by_id(context, resource)?;
        resources.push(spec)
    }

    let new_recipe = NewRecipe::new(&agent_id, &name, note.as_deref());

    let inserted_recipe: Recipe = diesel::insert_into(recipes::table)
        .values(new_recipe)
        .get_result(conn)?;

    for resource in recipe_resources {
        let new_recipe_resource = NewRecipeResource::new(inserted_recipe.id, resource);
        diesel::insert_into(recipe_resources::table)
            .values(new_recipe_resource)
            .execute(conn)?;
    }

    Ok(RecipeWithResources::new(inserted_recipe, resources))
}

pub fn set_recipe_processes(
    context: &Context,
    recipe_id: Uuid,
    recipe_template_ids: Vec<Uuid>,
    name: String,
    relations: Vec<RecipeProcessRelations>,
) -> FieldResult<()> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    conn.transaction::<_, FieldError, _>(|conn| {
        
        //check relations are present in recipe_template_ids
        for relation in relations {
            let RecipeProcessRelations {
                recipe_process_id,
                output_of,
            } = relation;

            let recipe_process_id_exists = recipe_template_ids
                .iter()
                .any(|recipe_id| *recipe_id == recipe_process_id);

            // Check if output_of exists in recipe_template_ids
            let output_of_exists = recipe_template_ids
                .iter()
                .any(|recipe_id| *recipe_id == output_of);

            if !recipe_process_id_exists || !output_of_exists {
                return Err(FieldError::new(
                    "Relation references invalid recipe process ID",
                    graphql_value!({ "code": "Relations Don't match" }),
                ));
            }
        }

        let mut recipes_res: Vec<RecipeProcess> = Vec::new();

        for recipe_template_id in recipe_template_ids {
            let new_recipe = NewRecipeProcess::new(&recipe_id, &recipe_template_id, &name);

            let inserted_recipe = diesel::insert_into(recipe_processes::table)
                .values(new_recipe)
                .get_result(conn)?;

            recipes_res.push(inserted_recipe);

            //get blacklist for each template
            let template_rules: BlacklistResponse = get_blacklists_by_template_id(&context, recipe_template_id)?;

            /*
            recipe_process_id -> successor
            output_of -> predecessor
             */

            //TODO: continue here

            







        }

        Ok(())
    })
}
