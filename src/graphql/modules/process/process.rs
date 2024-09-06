
use juniper::FieldResult;
use uuid::Uuid;

use crate::{graphql::context::Context, templates::recipe_template::RecipeTemplateWithRecipeFlows};

struct DefaultValues {
    field_id: Uuid,
    value: String
}
struct RecipeProcessWithRelation {
    recipe_process: RecipeTemplateWithRecipeFlows,
    output_of: RecipeTemplateWithRecipeFlows,
    default_values: Vec<DefaultValues>
}

/** Mutations */
pub fn create_recipe_processes(context: &Context, data: Vec<RecipeProcessWithRelation>) -> FieldResult<()> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    
    Ok(())
}
