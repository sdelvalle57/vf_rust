use crate::{
    db::schema::{recipe_resources, recipes},
    graphql::context::Context,
    recipe::recipe::{NewRecipe, NewRecipeResource, Recipe, RecipeResource, RecipeWithResources},
    resource_specification::ResourceSpecification,
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

use super::resource_specification::resource_specifications_by_id;

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
        let spec = resource_specifications_by_id(context, resource.resource_specification_id)?;
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
            let spec = resource_specifications_by_id(context, resource.resource_specification_id)?;
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
        let spec = resource_specifications_by_id(context, resource)?;
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
