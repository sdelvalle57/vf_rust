
use crate::{
    common::economic_resource::{EconomicResource, EconomicResourceWithSpec, NewEconomicResource}, db::schema::economic_resources, graphql::context::Context
};
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

use super::resource_specification::resource_specifications_by_agent;


/*** Queries */
pub fn economic_resources_by_specification_id(
    context: &Context,
    resource_specification_id: Uuid
) -> FieldResult<Vec<EconomicResource>> {
    let conn = &mut context.pool.get().expect("Failed to get DB connection from pool");

    let results = economic_resources::table
        .filter(economic_resources::resource_specification_id.eq(resource_specification_id))
        .load::<EconomicResource>(conn)?; 

    Ok(results)
}

pub fn economic_resources_by_agent(
    context: &Context,
    agent_id: Uuid
) -> FieldResult<Vec<EconomicResourceWithSpec>> {
    let resource_specifications_by_agent = resource_specifications_by_agent(&context, agent_id)?;

    let mut result: Vec<EconomicResourceWithSpec> = Vec::new();

    for spec in resource_specifications_by_agent {
        let economic_resources = economic_resources_by_specification_id(&context, spec.id)?;
        for economic_resource in economic_resources {
            let economic_resource_with_spec = EconomicResourceWithSpec::build(economic_resource, spec.clone());
            result.push(economic_resource_with_spec);
        }
    }

    Ok(result)
}

/*** Mutations */
pub fn create_economic_resource(
    context: &Context,
    resource_specification_id: Uuid, 
    name: String,
    note: Option<String>,
    accounting_quantity: i32,
    tracking_identifier: Option<String>,
    current_location: String,
    lot: Option<String>,
    contained_in: Option<Uuid>
) -> FieldResult<EconomicResource> {
    let conn = &mut context.pool.get().expect("Failed to get DB connection from pool");

    // Create the new resource specification instance
    let new_economic_resource = NewEconomicResource::new(
        &resource_specification_id, 
        &name, 
        note.as_deref(), 
        &accounting_quantity, 
        tracking_identifier.as_deref(), 
        &current_location, 
        lot.as_deref(), 
        contained_in.as_ref()
    ); 
        
    // Insert the new resource specification into the database
    let inserted_resource_spec = diesel::insert_into(economic_resources::table)
        .values(&new_economic_resource)
        .get_result(conn)?;

    Ok(inserted_resource_spec)

}
