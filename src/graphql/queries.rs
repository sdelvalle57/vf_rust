use crate::{
    agent::Agent, economic_resource::{EconomicResource, EconomicResourceWithSpec}, graphql::context::Context, recipe::recipe::RecipeWithResources, resource_specification::ResourceSpecification
};
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;
use super::modules::{agent, economic_resource, recipe, resource_specification};


pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {

    /*** Agents */
    fn all_agents(context: &Context) -> FieldResult<Vec<Agent>> {
        agent::all_agents(&context)
    }

    fn agent_by_id(context: &Context, agent_id: Uuid) -> FieldResult<Agent> {
        agent::agent_by_id(&context, agent_id)
    }

    
    /*** Resource Specifications */
    fn all_resource_specifications(context: &Context) -> FieldResult<Vec<ResourceSpecification>> {
        resource_specification::all_resource_specifications(&context)
    }

    fn resource_specifications_by_agent(
        context: &Context,
        agent_id: Uuid,
    ) -> FieldResult<Vec<ResourceSpecification>> {
        resource_specification::resource_specifications_by_agent(&context, agent_id)
    }

    fn resource_specifications_by_id(
        context: &Context,
        resource_specification_id: Uuid,
    ) -> FieldResult<ResourceSpecification> {
        resource_specification::resource_specifications_by_id(&context, resource_specification_id)
    }


    /*** Economic Resources */
    fn economic_resources_by_specification_id(
        context: &Context,
        resource_specification_id: Uuid
    ) -> FieldResult<Vec<EconomicResource>> {
        economic_resource::economic_resources_by_specification_id(&context, resource_specification_id)
    }

    fn economic_resources_by_agent_id(
        context: &Context,
        agent_id: Uuid
    ) -> FieldResult<Vec<EconomicResourceWithSpec>> {
        economic_resource::economic_resources_by_agent(&context, agent_id)
    }


    /*** Recipe */
    fn recipe_by_id(
        context: &Context,
        recipe_id: Uuid
    ) -> FieldResult<RecipeWithResources> {
        recipe::recipe_by_id(&context, recipe_id)
    }

    fn recipes_by_agent(
        context: &Context,
        agent_id: Uuid
    ) -> FieldResult<Vec<RecipeWithResources>> {
        recipe::recipes_by_agent(&context, agent_id)
    }

}
