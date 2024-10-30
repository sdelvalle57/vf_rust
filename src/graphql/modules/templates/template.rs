use crate::{
    db::schema::{
        map_templates, recipe_flow_template_data_fields, recipe_flow_template_group_data_fields,
        recipe_flow_templates, recipe_template_blacklists, recipe_templates,
        recipe_templates_access,
    },
    graphql::context::Context,
    templates::{
        self,
        map_template::{MapTemplate, MapTemplateResponse, NewMapTemplate, TemplateType},
        recipe_flow_template::{
            ActionType, EventType, NewRecipeFlowTemplate, RecipeFlowTemplate,
            RecipeFlowTemplateWithDataFields, RoleType,
        },
        recipe_flow_template_data_field::{
            FieldClass, FieldType, FlowThrough, NewRecipeFlowTemplateDataField,
            RecipeFlowTemplateDataField, RecipeFlowTemplateDataFieldInput,
        },
        recipe_flow_template_group_data_fields::{
            FieldGroupClass, NewRecipeFlowTemplateGroupDataField, RecipeFlowTemplateGroupDataField,
        },
        recipe_template::{NewRecipeTemplate, RecipeTemplate, RecipeTemplateWithRecipeFlows},
        recipe_template_access::{NewRecipeTemplateAccess, RecipeTemplateAccess},
        recipe_template_blacklist::{NewRecipeTemplateBlacklist, RecipeTemplateBlacklist},
    },
};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use juniper::{graphql_value, FieldError, FieldResult};
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateGroup {
    name: String,
    class: FieldGroupClass,
    fields: Vec<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateArg {
    event_type: EventType,
    role_type: RoleType,
    action: ActionType,
    data_fields: Vec<RecipeFlowTemplateDataFieldArg>,
    groups: Vec<RecipeFlowTemplateGroup>,
    identifier: String,
    interactions: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct FieldInheritance {
    identifier: String,
    field: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct RecipeFlowTemplateDataFieldArg {
    pub field_identifier: String,
    pub field_class: FieldClass,
    pub field: String,
    pub field_type: FieldType,
    pub note: Option<String>,
    pub required: bool,
    pub flow_through: Option<FlowThrough>,
    pub inherits: Option<FieldInheritance>,
    pub accept_default: bool,
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct MapTemplateBlacklist {
    pub recipe_template_id: Uuid,
    pub recipe_template_predecesor_id: Uuid,
}

#[derive(juniper::GraphQLObject)]
pub struct BlacklistResponse {
    pub successors: Vec<Uuid>,
    pub predecessors: Vec<Uuid>,
}

/** Queries */
fn get_recipe_flow_template_data_fields(
    context: &Context,
    recipe_flow_template: RecipeFlowTemplate,
) -> FieldResult<RecipeFlowTemplateWithDataFields> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut recipe_flow_remplate_data_fields =
        RecipeFlowTemplateWithDataFields::new(&recipe_flow_template);

    //get recipe flow template data fields by recipe flow template id
    let recipe_flow_template_data_fields: Vec<RecipeFlowTemplateDataField> =
        recipe_flow_template_data_fields::table
            .filter(
                recipe_flow_template_data_fields::recipe_flow_template_id
                    .eq(recipe_flow_template.id),
            )
            .load::<RecipeFlowTemplateDataField>(conn)?;

    for rftdf in &recipe_flow_template_data_fields {
        // If possible, adjust `try_into()` to accept a reference instead
        let recipe_flow_template_data_field_input: RecipeFlowTemplateDataFieldInput = rftdf
            .try_into() // Ensure this conversion works with a reference
            .map_err(|e| FieldError::new(e, juniper::Value::null()))?;

        recipe_flow_remplate_data_fields.add_data_field(recipe_flow_template_data_field_input);

        if let Some(group_id) = rftdf.group_id {
            let group = recipe_flow_template_group_data_fields::table
                .filter(recipe_flow_template_group_data_fields::id.eq(group_id))
                .first::<RecipeFlowTemplateGroupDataField>(conn)?;
            recipe_flow_remplate_data_fields.add_group(group);
        }
    }
    Ok(recipe_flow_remplate_data_fields)
}

fn get_recipe_template_with_flows(
    context: &Context,
    recipe_template: RecipeTemplate,
) -> FieldResult<RecipeTemplateWithRecipeFlows> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut recipe_template_with_recipe_flows: RecipeTemplateWithRecipeFlows =
        RecipeTemplateWithRecipeFlows::new(&recipe_template);

    //get recipe flow templates by recipe template id
    let recipe_flow_templates: Vec<RecipeFlowTemplate> = recipe_flow_templates::table
        .filter(recipe_flow_templates::recipe_template_id.eq(recipe_template.id))
        .load::<RecipeFlowTemplate>(conn)?;

    for rft in recipe_flow_templates {
        //create new instance of RecipeFlowTemplateWithDataFields
        let recipe_flow_remplate_data_fields = get_recipe_flow_template_data_fields(&context, rft)?;

        recipe_template_with_recipe_flows.add_recipe_flow(recipe_flow_remplate_data_fields)
    }

    Ok(recipe_template_with_recipe_flows)
}

pub fn get_map_templates(context: &Context) -> FieldResult<Vec<MapTemplateResponse>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let map_templates: Vec<MapTemplate> = map_templates::table.load::<MapTemplate>(conn)?;

    let mut res: Vec<MapTemplateResponse> = Vec::new();

    for map_template in map_templates {
        let blacklists: Vec<RecipeTemplateBlacklist> =
            get_blacklists_by_map_template(&context, map_template.id)?;

        let mut new_map_template = MapTemplateResponse::new(map_template, blacklists);

        let templates: Vec<RecipeTemplate> = recipe_templates::table
            .filter(recipe_templates::map_template_id.eq(new_map_template.map.id))
            .load::<RecipeTemplate>(conn)?;

        for template in templates {
            let recipe_template_with_recipe_flows =
                get_recipe_template_with_flows(&context, template)?;
            new_map_template.add_template(recipe_template_with_recipe_flows);
        }

        res.push(new_map_template);
    }

    Ok(res)
}

pub fn get_map_template_by_id(context: &Context, map_id: Uuid) -> FieldResult<MapTemplateResponse> {
    let conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> =
        &mut context
            .pool
            .get()
            .expect("Failed to get DB connection from pool");

    let map_template: MapTemplate = map_templates::table
        .filter(map_templates::id.eq(map_id))
        .first::<MapTemplate>(conn)?;

    let blacklists: Vec<RecipeTemplateBlacklist> =
        get_blacklists_by_map_template(&context, map_template.id)?;

    let mut new_map_template = MapTemplateResponse::new(map_template, blacklists);

    let templates: Vec<RecipeTemplate> = recipe_templates::table
        .filter(recipe_templates::map_template_id.eq(new_map_template.map.id))
        .load::<RecipeTemplate>(conn)?;

    for template in templates {
        let recipe_template_with_recipe_flows = get_recipe_template_with_flows(&context, template)?;
        new_map_template.add_template(recipe_template_with_recipe_flows);
    }

    Ok(new_map_template)
}

pub fn get_template_by_id(
    context: &Context,
    template_id: Uuid,
) -> FieldResult<RecipeTemplateWithRecipeFlows> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let recipe: RecipeTemplate = recipe_templates::table
        .filter(recipe_templates::id.eq(template_id))
        .first::<RecipeTemplate>(conn)?;

    let res = get_recipe_template_with_flows(&context, recipe)?;

    Ok(res)
}

pub fn get_templates_access_by_agent(
    context: &Context,
    agent_id: Uuid,
) -> FieldResult<Vec<RecipeTemplateWithRecipeFlows>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let accesses: Vec<RecipeTemplateAccess> = recipe_templates_access::table
        .filter(recipe_templates_access::agent_id.eq(agent_id))
        .load::<RecipeTemplateAccess>(conn)?;

    let mut res: Vec<RecipeTemplateWithRecipeFlows> = Vec::new();

    for a in accesses {
        let template_id = a.recipe_template_id;
        let mut recipe = get_template_by_id(context, template_id)?;
        res.push(recipe)
    }

    Ok(res)
}

fn get_blacklists_by_map_template(
    context: &Context,
    map_template_id: Uuid,
) -> FieldResult<Vec<RecipeTemplateBlacklist>> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let blacklists: Vec<RecipeTemplateBlacklist> = recipe_template_blacklists::table
        .filter(recipe_template_blacklists::map_template_id.eq(map_template_id))
        .load::<RecipeTemplateBlacklist>(conn)?;

    Ok(blacklists)
}

fn get_recursive_templates(context: &Context, template_id: Uuid) -> FieldResult<Vec<Uuid>> {
    let mut res: Vec<Uuid> = Vec::new();
    let mut current_template_id = Some(template_id);

    // Traverse templates recursively
    while let Some(template_id) = current_template_id {
        // Get the current template by ID
        let template = get_template_by_id(context, template_id)?;

        // Add the current template ID to the result vector
        res.push(template_id);

        // Update current_template_id with `overriden_by` if it exists, otherwise end loop
        current_template_id = template.overriden_by;
    }

    Ok(res)
}

fn get_template_first_version(context: &Context, template_id: Uuid) -> FieldResult<Uuid> {
    let mut current_template_id = Some(template_id);

    while let Some(template_id) = current_template_id {
        // Get the current template by ID
        let template = get_template_by_id(context, template_id)?;

        // If `overriden_by` is None, return the current template ID
        if template.overriden_by.is_none() {
            return Ok(template_id);
        }

        // Update current_template_id with `overriden_by` for next iteration
        current_template_id = template.overriden_by;
    }

    // This should not be reached if the loop is correct, but we return an error for safety
    Err(FieldError::new(
        "No valid template found",
        graphql_value!({ "code": "Invalid Template" }),
    ))
}

pub fn get_blacklists_by_template_id(
    context: &Context,
    template_id: Uuid,
) -> FieldResult<BlacklistResponse> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let mut predecessors: Vec<Uuid> = Vec::new();
    let mut successors: Vec<Uuid> = Vec::new();

    let template_first_version = get_template_first_version(&context, template_id)?;

    let blacklists: Vec<RecipeTemplateBlacklist> = recipe_template_blacklists::table
        .filter(
            recipe_template_blacklists::recipe_template_id
                .eq(template_first_version)
                .or(recipe_template_blacklists::recipe_template_predecesor_id.eq(template_first_version)),
        )
        .load::<RecipeTemplateBlacklist>(conn)?;

    for blacklist in blacklists {
        if blacklist.recipe_template_id == template_first_version {
            predecessors.push(blacklist.recipe_template_predecesor_id);
        } else if blacklist.recipe_template_predecesor_id == template_first_version {
            successors.push(blacklist.recipe_template_id)
        }
    }

    let res = BlacklistResponse {
        predecessors,
        successors,
    };

    Ok(res)
}

pub fn create_map_template(
    context: &Context,
    name: String,
    type_: TemplateType,
) -> FieldResult<MapTemplate> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    conn.transaction::<_, FieldError, _>(|conn| {
        let new_map_template = NewMapTemplate::new(&name, &type_);

        let inserted_map_template: MapTemplate = diesel::insert_into(map_templates::table)
            .values(&new_map_template)
            .get_result(conn)?;

        Ok(inserted_map_template)
    })
}

pub fn create_recipe_template(
    context: &Context,
    map_template_id: Uuid,
    identifier: String,
    name: String,
    recipe_flow_template_args: Vec<RecipeFlowTemplateArg>,
    commitment: Option<ActionType>,
    fulfills: Option<String>,
    trigger: Option<ActionType>,
    version: i32,
    overrides: Option<Uuid>,
    created_by: Option<Uuid>,
) -> FieldResult<RecipeTemplateWithRecipeFlows> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    // Start a transaction
    conn.transaction::<_, FieldError, _>(|conn| {
        // Create the new recipe template
        let fulfills_id = if let Some(process_identifier) = fulfills {
            // Attempt to get the recipe by identifier
            let recipe: RecipeTemplate = recipe_templates::table
                .filter(recipe_templates::identifier.eq(process_identifier))
                .first::<RecipeTemplate>(conn)?;

            // Return the recipe's id wrapped in Some
            Some(recipe.id)
        } else {
            // If fulfills is None, return None
            None
        };

        let new_template = NewRecipeTemplate::new(
            &map_template_id,
            &identifier,
            &name,
            commitment.as_ref(),
            fulfills_id.as_ref(),
            trigger.as_ref(),
            version,
            None,
            created_by.as_ref(),
        );

        let inserted_template: RecipeTemplate = diesel::insert_into(recipe_templates::table)
            .values(&new_template)
            .get_result(conn)
            .map_err(|e| FieldError::new(e, juniper::Value::null()))?; // Map diesel::result::Error to FieldError

        //check if this template overrides another one
        if let Some(template_id) = overrides {
            let old_template = recipe_templates::table
                .filter(recipe_templates::id.eq(template_id))
                .first::<RecipeTemplate>(conn)?;

            if let Some(_) = old_template.overriden_by {
                return Err(FieldError::new(
                    "Cannot override template, already overriden",
                    graphql_value!({ "code": "ALREADY_OVERRIDEN" }),
                ));
            }

            diesel::update(
                recipe_templates::table.filter(recipe_templates::id.eq(old_template.id)),
            )
            .set(recipe_templates::overriden_by.eq(inserted_template.id))
            .execute(conn)?;
        }

        // Initialize the result struct
        let mut res: RecipeTemplateWithRecipeFlows =
            RecipeTemplateWithRecipeFlows::new(&inserted_template);

        // Iterate over each `RecipeFlowTemplateArg`
        for r in recipe_flow_template_args {
            // Create and insert a new recipe flow template
            let new_recipe_flow_template = NewRecipeFlowTemplate::new(
                &inserted_template.id,
                &r.event_type,
                &r.role_type,
                &r.action,
                &r.identifier,
                r.interactions.as_ref(),
            );

            let inserted_recipe_flow_template: RecipeFlowTemplate =
                diesel::insert_into(recipe_flow_templates::table)
                    .values(&new_recipe_flow_template)
                    .get_result::<RecipeFlowTemplate>(conn)?;

            // Initialize `RecipeFlowTemplateWithDataFields` struct
            let mut recipe_flow_res =
                RecipeFlowTemplateWithDataFields::new(&inserted_recipe_flow_template);

            let mut groups: Vec<(Uuid, Vec<String>)> = Vec::new();

            for group in r.groups {
                let new_group = NewRecipeFlowTemplateGroupDataField::new(&group.name, &group.class);

                let inserted_group: RecipeFlowTemplateGroupDataField =
                    diesel::insert_into(recipe_flow_template_group_data_fields::table)
                        .values(new_group)
                        .get_result::<RecipeFlowTemplateGroupDataField>(conn)?;

                let separated_fields: Vec<String> = group
                    .fields
                    .iter()
                    .flat_map(|field| field.split(", ").map(String::from).collect::<Vec<String>>()) // Split by ", " and collect into a Vec<String>
                    .collect();

                groups.push((inserted_group.id, separated_fields));
            }

            // Iterate over each data field and add it to the recipe flow
            for rd in r.data_fields {
                let group_id = groups
                    .iter()
                    .find(|g| g.1.contains(&rd.field_identifier))
                    .map(|group| group.0);

                let inherits: Option<Uuid> = if let Some(inherits) = rd.inherits {
                    //search for the recipe flow template with the identifier
                    let recipe_flow_template: RecipeFlowTemplate = recipe_flow_templates::table
                        .filter(recipe_flow_templates::recipe_template_id.eq(&inserted_template.id))
                        .filter(recipe_flow_templates::identifier.eq(inherits.identifier))
                        .first::<RecipeFlowTemplate>(conn)?;

                    let field: RecipeFlowTemplateDataField =
                        recipe_flow_template_data_fields::table
                            .filter(
                                recipe_flow_template_data_fields::recipe_flow_template_id
                                    .eq(recipe_flow_template.id),
                            )
                            .filter(
                                recipe_flow_template_data_fields::field_identifier
                                    .eq(inherits.field),
                            )
                            .first::<RecipeFlowTemplateDataField>(conn)?;

                    Some(field.id)
                } else {
                    None
                };

                let new_recipe_flow_template_data_field = NewRecipeFlowTemplateDataField::new(
                    &inserted_recipe_flow_template.id,
                    group_id.as_ref(),
                    &rd.field_identifier,
                    &rd.field_class,
                    &rd.field,
                    &rd.field_type,
                    rd.note.as_deref(),
                    &rd.required,
                    rd.flow_through.as_ref(),
                    inherits.as_ref(),
                    &rd.accept_default,
                );

                let inserted_recipe_flow_template_data_field: RecipeFlowTemplateDataField =
                    diesel::insert_into(recipe_flow_template_data_fields::table)
                        .values(new_recipe_flow_template_data_field)
                        .get_result(conn)
                        .map_err(|e| FieldError::new(e, juniper::Value::null()))?; // Map diesel::result::Error to FieldError

                let recipe_flow_template_data_field_input: RecipeFlowTemplateDataFieldInput =
                    (&inserted_recipe_flow_template_data_field)
                        .try_into()
                        .map_err(|e| FieldError::new(e, juniper::Value::null()))?;

                // Add the data field to the recipe flow
                recipe_flow_res.add_data_field(recipe_flow_template_data_field_input);

                if let Some(group_id) = group_id {
                    let group: RecipeFlowTemplateGroupDataField =
                        recipe_flow_template_group_data_fields::table
                            .filter(recipe_flow_template_group_data_fields::id.eq(group_id))
                            .first::<RecipeFlowTemplateGroupDataField>(conn)?;

                    recipe_flow_res.add_group(group);
                }
            }
            res.add_recipe_flow(recipe_flow_res);
        }

        Ok(res)
    })
}

pub fn set_map_template_blacklists(
    context: &Context,
    map_template_id: Uuid,
    selected_template_id: Uuid,
    blacklists: Vec<MapTemplateBlacklist>,
) -> FieldResult<MapTemplateResponse> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    conn.transaction::<_, FieldError, _>(|conn| {
        //remove all related to selected_template_id
        diesel::delete(recipe_template_blacklists::table)
            .filter(
                recipe_template_blacklists::map_template_id
                    .eq(map_template_id)
                    .and(
                        recipe_template_blacklists::recipe_template_id
                            .eq(selected_template_id)
                            .or(recipe_template_blacklists::recipe_template_predecesor_id
                                .eq(selected_template_id)),
                    ),
            )
            .execute(conn)?;

        for blacklist in blacklists {
            let template_id = blacklist.recipe_template_id;
            let predecessor_id = blacklist.recipe_template_predecesor_id;

            let new_blacklist =
                NewRecipeTemplateBlacklist::new(&map_template_id, &template_id, &predecessor_id);
            diesel::insert_into(recipe_template_blacklists::table)
                .values(&new_blacklist)
                .execute(conn)?;
        }
        let map_template = get_map_template_by_id(context, map_template_id)?;
        Ok(map_template)
    })
}

pub fn assign_template_to_agent(
    context: &Context,
    recipe_template_id: Uuid,
    agent_id: Uuid,
) -> FieldResult<RecipeTemplateAccess> {
    let conn = &mut context
        .pool
        .get()
        .expect("Failed to get DB connection from pool");

    let existing_access: Result<RecipeTemplateAccess, DieselError> = recipe_templates_access::table
        .filter(recipe_templates_access::recipe_template_id.eq(recipe_template_id))
        .filter(recipe_templates_access::agent_id.eq(agent_id))
        .first::<RecipeTemplateAccess>(conn);

    // If it exists, return an error
    if let Ok(_) = existing_access {
        return Err(FieldError::new(
            "Template access already exists for this agent.",
            graphql_value!({ "code": "ALREADY_EXISTS" }),
        ));
    }

    // If it doesn't exist, insert a new record
    let new_access = NewRecipeTemplateAccess::new(&agent_id, &recipe_template_id);

    let res: RecipeTemplateAccess = diesel::insert_into(recipe_templates_access::table)
        .values(&new_access)
        .get_result::<RecipeTemplateAccess>(conn)?;

    Ok(res)
}

#[cfg(test)]
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
    fn logic_delete() {
        // Use the test pool instead of `context.pool`
        let pool = get_test_pool();
        let conn = &mut pool.get().expect("Failed to get DB connection from pool");

        let result = conn.transaction::<_, DieselError, _>(|conn| {
            //Delete recipe_flow_template_data_fields
            diesel::delete(recipe_flow_template_data_fields::table).execute(conn)?;
            let remaining_rows: i64 = recipe_flow_template_data_fields::table
                .count()
                .get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipe_flow_templates
            diesel::delete(recipe_flow_templates::table).execute(conn)?;
            let remaining_rows: i64 = recipe_flow_templates::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipe_flow_template_group_data_fields
            diesel::delete(recipe_flow_template_group_data_fields::table).execute(conn)?;
            let remaining_rows: i64 = recipe_flow_template_group_data_fields::table
                .count()
                .get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipe_templates_access
            diesel::delete(recipe_templates_access::table).execute(conn)?;
            let remaining_rows: i64 = recipe_templates_access::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete recipe_templates
            diesel::delete(recipe_templates::table).execute(conn)?;
            let remaining_rows: i64 = recipe_templates::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            //Delete map_templates
            diesel::delete(map_templates::table).execute(conn)?;
            let remaining_rows: i64 = map_templates::table.count().get_result(conn)?;
            assert_eq!(remaining_rows, 0); // Ensure all rows are deleted

            Ok(())
        });

        assert!(result.is_ok(), "Transaction failed: {:?}", result);
    }
}
