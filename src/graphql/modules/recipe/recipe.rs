use crate::{
    common::resource_specification::ResourceSpecification,
    db::schema::{recipe_process_relations, recipe_processes, recipe_resources, recipes},
    graphql::{
        context::Context,
        modules::{
            common::resource_specification::resource_specification_by_id,
            templates::template::{check_blacklist_connection, get_template_first_version},
        },
    },
    recipe::{
        process::process::{
            NewProcessRelation, NewRecipeProcess, ProcessRelation, RecipeProcess,
            RecipeProcessResponse,
        },
        recipe::{NewRecipe, NewRecipeResource, Recipe, RecipeResource, RecipeWithResources},
    },
};
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult, GraphQLInputObject};
use uuid::Uuid;

#[derive(GraphQLInputObject)]
pub struct RecipeProcessRelation {
    pub template_id: Uuid,
    pub template_predecessor_id: Uuid,
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
        let spec = resource_specification_by_id(&context, resource.resource_specification_id)?;
        resources.push(spec)
    }

    let recipe_relations = recipe_relations(&context, recipe_id)?;

    Ok(RecipeWithResources::new(
        recipe,
        resources,
        recipe_relations,
    ))
}

fn recipe_relations(context: &Context, recipe_id: Uuid) -> FieldResult<Vec<ProcessRelation>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let res: Vec<ProcessRelation> = recipe_process_relations::table
        .filter(recipe_process_relations::recipe_id.eq(recipe_id))
        .load::<ProcessRelation>(conn)?;

    Ok(res)
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

        let recipe_relations = recipe_relations(&context, recipe.id)?;

        recipes_response.push(RecipeWithResources::new(
            recipe,
            resources,
            recipe_relations,
        ))
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

    Ok(RecipeWithResources::new(
        inserted_recipe,
        resources,
        Vec::new(),
    ))
}


//TODO: probably remove template_ids and iterate over only relations
pub fn set_recipe_processes(
    context: &Context,
    recipe_id: Uuid,
    template_ids: Vec<Uuid>,
    name: String,
    relations: Vec<RecipeProcessRelation>,
) -> FieldResult<Vec<RecipeProcessResponse>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    conn.transaction::<_, FieldError, _>(|conn| {
        let mut res: Vec<RecipeProcessResponse> = Vec::new();

        for template_id in template_ids {
            let new_recipe = NewRecipeProcess::new(&recipe_id, &template_id, &name);

            let inserted_recipe = diesel::insert_into(recipe_processes::table)
                .values(new_recipe)
                .get_result(conn)?;

            let mut recipe_res = RecipeProcessResponse::new(inserted_recipe);

            //get all predecessors
            let filtered_relations: Vec<&RecipeProcessRelation> = relations
                .iter()
                .filter(|r| r.template_id == template_id)
                .collect();

            for relation in filtered_relations {
                let is_not_blacklisted = can_connect(&context, &relation)?;

                if is_not_blacklisted {
                    let new_relation = NewProcessRelation::new(
                        &recipe_id,
                        &relation.template_id,
                        &relation.template_predecessor_id,
                    );
                    let inserted_relation: ProcessRelation =
                        diesel::insert_into(recipe_process_relations::table)
                            .values(new_relation)
                            .get_result::<ProcessRelation>(conn)?;
                    recipe_res.add_predecessor(inserted_relation.predecessor);
                } else {
                    return Err(FieldError::new(
                        "Invalid Template Relation",
                        graphql_value!({ "code": "Invalid Template" }),
                    ));
                }
            }
            res.push(recipe_res);
        }
        Ok(res)
    })
}

fn can_connect(context: &Context, relation: &RecipeProcessRelation) -> FieldResult<bool> {
    let RecipeProcessRelation {
        template_id,
        template_predecessor_id,
    } = relation;

    let recipe_process_id_first_version = get_template_first_version(&context, template_id)?;
    let predecessor_first_version = get_template_first_version(&context, template_predecessor_id)?;

    let res = check_blacklist_connection(
        &context,
        recipe_process_id_first_version,
        predecessor_first_version,
    )?;

    Ok(res)
}

mod tests {
    use std::env;

    use super::*;
    use diesel::prelude::*;
    use diesel::r2d2;
    use diesel::r2d2::ConnectionManager;
    use diesel::result::Error as DieselError;
    use diesel::PgConnection;

    use crate::db::schema::{
        recipe_flow_template_data_fields, recipe_flow_template_group_data_fields,
        recipe_flow_templates, recipe_templates_access,
    };

    // Initialize the pool for testing purposes
    fn get_test_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
        let database_url = "postgres://value_flows:valueflows@localhost/vf26";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    #[test]
    fn logic_delete_recipes() {
        // Use the test pool instead of `context.pool`
        let pool = get_test_pool();
        let conn = &mut pool.get().expect("Failed to get DB connection from pool");

        let result = conn.transaction::<_, DieselError, _>(|conn| {
            //Delete recipe_process_relations
            diesel::delete(recipe_process_relations::table).execute(conn)?;
            let remaining_rows: i64 = recipe_process_relations::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipe_processes
            diesel::delete(recipe_processes::table).execute(conn)?;
            let remaining_rows: i64 = recipe_processes::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipe_resources
            diesel::delete(recipe_resources::table).execute(conn)?;
            let remaining_rows: i64 = recipe_resources::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipe_templates_access
            diesel::delete(recipe_templates_access::table).execute(conn)?;
            let remaining_rows: i64 = recipe_templates_access::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipes
            diesel::delete(recipes::table).execute(conn)?;
            let remaining_rows: i64 = recipes::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            Ok(())
        });

        assert!(result.is_ok(), "Transaction failed: {:?}", result);
    }
}
