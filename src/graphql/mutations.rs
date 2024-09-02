use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use crate::{
    common::{
        agent::Agent, economic_resource::EconomicResource,
        resource_specification::{ResourceSpecification, ResourceType},
    }, graphql::context::Context, recipe::recipe::RecipeWithResources, templates::recipe_template::{RecipeTemplateType, RecipeTemplateWithRecipeFlows}
};

use super::modules::{common::{agent, economic_resource, resource_specification}, recipe::recipe::create_recipe, templates::template::{self, RecipeFlowTemplateArg}};

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    /*** Agents */
    fn create_agent(context: &Context, name: String, note: Option<String>) -> FieldResult<Agent> {
        agent::create_agent(&context, name, note)
    }

    /** Resource Specifications */
    fn create_resource_specification(
        context: &Context,
        agent_id: Uuid,
        name: String,
        note: Option<String>,
        resource_type: ResourceType,
        unit_of_measure: String,
    ) -> FieldResult<ResourceSpecification> {
        resource_specification::create_resource_specification(
            &context,
            agent_id,
            name,
            note,
            resource_type,
            unit_of_measure,
        )
    }

    /** Economic Resource */
    fn create_economic_resource(
        context: &Context,
        resource_specification_id: Uuid,
        name: String,
        note: Option<String>,
        accounting_quantity: i32,
        tracking_identifier: Option<String>,
        current_location: String,
        lot: Option<String>,
        contained_in: Option<Uuid>,
    ) -> FieldResult<EconomicResource> {
        economic_resource::create_economic_resource(
            &context,
            resource_specification_id,
            name,
            note,
            accounting_quantity,
            tracking_identifier,
            current_location,
            lot,
            contained_in,
        )
    }

    /** Recipe Templates */
    fn create_recipe_template(
        context: &Context,
        name: String,
        recipe_template_type: RecipeTemplateType,
        recipe_flow_template_args: Vec<RecipeFlowTemplateArg>,
    ) -> FieldResult<RecipeTemplateWithRecipeFlows> {
        template::create_recipe_template(context, name, recipe_template_type, recipe_flow_template_args)
    }

    /** Recipe */
    fn create_recipe(
        context: &Context,
        agent_id: Uuid,
        name: String,
        note: Option<String>,
        recipe_resources: Vec<Uuid>,
    ) -> FieldResult<RecipeWithResources> {
        create_recipe(&context, agent_id, name, note, recipe_resources)
    }

    // /** Process */
    // fn create_process(
    //     context: &Context,
    //     recipe_id: Uuid,
    //     name: String,
    //     note: Option<String>,
    //     output_of: Option<Uuid>,
    // ) -> FieldResult<ProcessWithRecipe> {
    //     create_process(&context, recipe_id, name, note, output_of)
    // }
}
