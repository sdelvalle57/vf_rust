
use crate::{
    common::resource_specification::{NewResourceSpecification, ResourceSpecification, ResourceType}, db::schema::resource_specifications, graphql::context::Context
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

pub fn all_resource_specifications(context: &Context) -> FieldResult<Vec<ResourceSpecification>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let results = resource_specifications::table.load::<ResourceSpecification>(conn)?; // The `load` method is now available

    Ok(results)
}

pub fn resource_specifications_by_agent(
    context: &Context,
    agent_id: Uuid,
) -> FieldResult<Vec<ResourceSpecification>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let results = resource_specifications::table
        .filter(resource_specifications::agent_id.eq(agent_id))
        .load::<ResourceSpecification>(conn)?;

    Ok(results)
}

pub fn resource_specification_by_id(
    context: &Context,
    resource_specification_id: Uuid,
) -> FieldResult<ResourceSpecification> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let results = resource_specifications::table
        .filter(resource_specifications::id.eq(resource_specification_id))
        .first::<ResourceSpecification>(conn)?;

    Ok(results)
}


/*** Mutations */

pub fn create_resource_specification(
    context: &Context,
    agent_id: Uuid, 
    name: String,
    note: Option<String>,
    resource_type: ResourceType,
    unit_of_measure: String
) -> FieldResult<ResourceSpecification> {
    let conn = &mut context.pool.get().expect("Failed to get DB connection from pool");

    let new_resource_spec = NewResourceSpecification::new(
        &agent_id, 
        &name, 
        note.as_deref(), 
        &resource_type, 
        &unit_of_measure
    );

    // Insert the new resource specification into the database
    let inserted_resource_spec = diesel::insert_into(resource_specifications::table)
        .values(&new_resource_spec)
        .get_result(conn)?;

    Ok(inserted_resource_spec)
}