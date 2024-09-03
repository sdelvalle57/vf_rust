use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::Insertable,
    serialize::{self, IsNull, Output, ToSql},
};
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{
    db::schema::{process_flow_data_field_queries, sql_types::QueryTypeEnum},
    graphql::modules::templates::template::RecipeFlowTemplateDataFieldArg,
};

use super::recipe_flow_template_data_field::FieldValue;

// Assuming QueryTypeEnum is defined as a custom SQL type
#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum, Clone)]
#[diesel(sql_type = QueryTypeEnum)]
pub enum QueryType {
    Select,
}

// Implement ToSql for QueryType
impl ToSql<QueryTypeEnum, Pg> for QueryType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            QueryType::Select => out.write_all(b"Select")?,
            // Handle other variants
        }
        Ok(IsNull::No)
    }
}

// Implement FromSql for QueryType
impl FromSql<QueryTypeEnum, Pg> for QueryType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Select" => Ok(QueryType::Select),
            // Handle other variants
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct QueryCondition<'a> {
    pub field: &'a str,
    pub value: &'a str,
    pub operator: &'a str, // E.g., '=', '>', '<', 'LIKE', etc.
}

#[derive(Insertable)]
#[diesel(table_name = process_flow_data_field_queries)]
pub struct NewDBQuery<'a> {
    pub query_type: &'a QueryType,
    pub table_name: &'a str,
    pub fields: JsonValue,             // Serialize Vec<&str> into JSON
    pub conditions: Option<JsonValue>, // Optionally serialize Vec<QueryCondition> into JSON
    pub additional_clauses: Option<&'a str>,
}

impl<'a> NewDBQuery<'a> {
    fn new(
        query_type: &'a QueryType,
        table_name: &'a str,
        fields: Vec<&'a str>,
        conditions: Option<Vec<QueryCondition>>,
        additional_clauses: Option<&'a str>,
    ) -> Self {
        NewDBQuery {
            query_type,
            table_name,
            fields: serde_json::json!(fields), // Convert fields to JSON
            conditions: conditions.map(|conds| serde_json::to_value(conds).unwrap()), // Convert conditions to JSON
            additional_clauses,
        }
    }

    pub fn build(data_field: &RecipeFlowTemplateDataFieldArg, agent_id: &Uuid) -> Option<Self> {
        match data_field.field_value {
            FieldValue::Product => {
                let table_name = "agents";
                let fields = vec!["*"]; // Change from ' to "
                let agent_id_str = agent_id.to_string();
                
                let conditions = vec![QueryCondition {
                    field: "id",
                    value: &agent_id_str,
                    operator: "=",
                }];
                let additional_clauses = None;

                // Pass the correct QueryType variant
                let query = NewDBQuery::new(
                    &QueryType::Select, // Use QueryType::Select or the appropriate variant
                    table_name,
                    fields,
                    Some(conditions),
                    additional_clauses,
                );

                Some(query)
            }
            _ => None, // Fallback case
        }
    }

    pub fn to_raw_sql(&self) -> String {
        // Extract fields array and join with commas
        let fields_str = if let Some(fields_array) = self.fields.as_array() {
            fields_array
                .iter()
                .filter_map(|field| field.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        } else {
            String::new() // Or handle it as an error, depending on your use case
        };

        // Construct the base query
        let base_query = match self.query_type {
            QueryType::Select => format!("SELECT {} FROM {}", fields_str, self.table_name),
        };

        // Construct the WHERE clause if conditions are present
        let conditions_query = if let Some(conditions_json) = &self.conditions {
            if let Some(conditions_array) = conditions_json.as_array() {
                let conditions_str: Vec<String> = conditions_array
                    .iter()
                    .filter_map(|cond| {
                        if let (Some(field), Some(operator), Some(value)) = (
                            cond.get("field").and_then(JsonValue::as_str),
                            cond.get("operator").and_then(JsonValue::as_str),
                            cond.get("value").and_then(JsonValue::as_str),
                        ) {
                            Some(format!("{} {} '{}'", field, operator, value))
                        } else {
                            None
                        }
                    })
                    .collect();
                format!(" WHERE {}", conditions_str.join(" AND "))
            } else {
                String::new() // Or handle as an error if conditions are expected
            }
        } else {
            String::new()
        };

        // Add any additional clauses like ORDER BY or LIMIT
        let additional_clauses_query = self.additional_clauses.unwrap_or_default();

        // Combine all parts to form the final query
        format!(
            "{}{}{}",
            base_query, conditions_query, additional_clauses_query
        )
    }
}
