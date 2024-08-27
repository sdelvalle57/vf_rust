use crate::{
    db::schema::processes, 
    graphql::context::Context, 
    recipe::process::{NewProcess, Process, ProcessWithRecipe}, 
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

use super::recipe::recipe_by_id;

pub fn processes_by_recipe_id(
    context: &Context,
    recipe_id: Uuid
) -> FieldResult<Vec<Process>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let processes: Vec<Process> = processes::table
        .filter(processes::recipe_id.eq(recipe_id))
        .load::<Process>(conn)?;

    Ok(processes)
} 

/** Mutations */
pub fn create_process(
    context: &Context,
    recipe_id: Uuid,
    name: String,
    note: Option<String>,
    output_of: Option<Uuid>,
    template_id: Uuid
) -> FieldResult<ProcessWithRecipe> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let new_process = NewProcess::new(
        &recipe_id, 
        &name, 
        note.as_deref(), 
        output_of.as_ref(),
        &template_id
    );

    let inserted_process: Process = diesel::insert_into(processes::table)
        .values(new_process)
        .get_result(conn)?;

    let recipe = recipe_by_id(&context, recipe_id)?;

    let process = ProcessWithRecipe::new(inserted_process, recipe);

    Ok(process)
}
