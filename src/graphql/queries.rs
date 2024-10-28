use crate::{
    common::{
        agent::{Agent, AgentWithLocations}, economic_resource::{EconomicResource, EconomicResourceWithSpec}, location::Location, resource_specification::ResourceSpecification
    },
    graphql::context::Context,
    recipe::recipe::RecipeWithResources,
    templates::{map_template::MapTemplateResponse, recipe_template::RecipeTemplateWithRecipeFlows},
};
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use super::modules::{
    common::{agent, economic_resource, location, resource_specification}, 
    // process::process::{self, RecipeProcessesResponse}, 
    recipe::recipe, templates::template
};

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

    fn agents_with_location(context: &Context) -> FieldResult<Vec<AgentWithLocations>> {
        agent::agents_with_location(&context)
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

    fn resource_specification_by_id(
        context: &Context,
        resource_specification_id: Uuid,
    ) -> FieldResult<ResourceSpecification> {
        resource_specification::resource_specification_by_id(&context, resource_specification_id)
    }

    /*** Economic Resources */
    fn economic_resources_by_specification_id(
        context: &Context,
        resource_specification_id: Uuid,
    ) -> FieldResult<Vec<EconomicResource>> {
        economic_resource::economic_resources_by_specification_id(
            &context,
            resource_specification_id,
        )
    }

    fn economic_resources_by_agent_id(
        context: &Context,
        agent_id: Uuid,
    ) -> FieldResult<Vec<EconomicResourceWithSpec>> {
        economic_resource::economic_resources_by_agent(&context, agent_id)
    }

    /** Get Map Templates */
    fn get_map_templates(context: &Context) -> FieldResult<Vec<MapTemplateResponse>> {
        template::get_map_templates(context)
    }

    fn get_map_template_by_id(context: &Context, map_id: Uuid) -> FieldResult<MapTemplateResponse> {
        template::get_map_template_by_id(context, map_id)
    }

    /** Recipe Templates */

    fn get_template_by_id(context: &Context, template_id: Uuid) -> FieldResult<RecipeTemplateWithRecipeFlows> {
        template::get_template_by_id(context, template_id)
    }

    /** Recipe Template Access */
    fn get_templates_access_by_agent(
        context: &Context,
        agent_id: Uuid,
    ) -> FieldResult<Vec<RecipeTemplateWithRecipeFlows>> {
        template::get_templates_access_by_agent(context, agent_id)
    }

    /*** Recipe */
    fn recipe_by_id(context: &Context, recipe_id: Uuid) -> FieldResult<RecipeWithResources> {
        recipe::recipe_by_id(&context, recipe_id)
    }

    /** Locations */
    fn locations_by_agent(context: &Context, agent_id: Uuid) -> FieldResult<Vec<Location>> {
        location::locations_by_agent(&context, agent_id)
    }

    fn recipes_by_agent(
        context: &Context,
        agent_id: Uuid,
    ) -> FieldResult<Vec<RecipeWithResources>> {
        recipe::recipes_by_agent(&context, agent_id)
    }


    // fn get_recipe_processes(
    //     context: &Context,
    //     recipe_id: Uuid
    // ) -> FieldResult<RecipeProcessesResponse> {
    //     process::get_recipe_processes(context, recipe_id)
    // }
}
