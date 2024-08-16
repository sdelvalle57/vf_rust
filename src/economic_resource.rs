use chrono::{DateTime, Utc};

use crate::resource_specification::ResourceSpecification;

pub enum ResourceType {
    PRODUCT,
    RESOURCE,
    ASSET,
}

pub struct EconomicResource {
    pub id: String,
    pub resource_specification: ResourceSpecification,
    pub name: String,
    pub note: String,
    pub accounting_quantity: f64,
    pub on_hand_quantity: f64,
    pub unit_of_measure: String,
    pub tracking_identifier: Option<String>,
    pub current_location: String,
    pub lot: String,
    pub contained_in: Option<ResourceSpecification>,
    pub resource_type: ResourceType,
    pub created_at: DateTime<Utc>,
}

impl EconomicResource {
    pub fn new(
        id: &str,
        resource_specification: ResourceSpecification,
        name: &str,
        note: &str,
        accounting_quantity: f64,
        unit_of_measure: String,
        tracking_identifier: Option<String>,
        current_location: String,
        lot: String,
        contained_in: Option<ResourceSpecification>,
        resource_type: ResourceType,
    ) -> EconomicResource {
        EconomicResource {
            id: id.to_string(),
            resource_specification,
            name: name.to_string(),
            note: note.to_string(),
            accounting_quantity,
            on_hand_quantity: accounting_quantity,
            unit_of_measure: unit_of_measure.to_string(),
            tracking_identifier,
            current_location: current_location.to_string(),
            lot: lot.to_string(),
            contained_in,
            resource_type,
            created_at: Utc::now()
        }
    }
}
