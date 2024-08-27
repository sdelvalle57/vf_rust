use crate::{
    db::schema::processes, 
    graphql::context::Context, 
    recipe::process::{NewProcess, Process, ProcessWithRecipe}, 
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

/** Mutations */
pub fn create_template(
    context: &Context,
    name: String,
) -> FieldResult<ProcessWithRecipe> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let new_template = NewTemplate::new(name);

    

}