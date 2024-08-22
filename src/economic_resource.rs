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
    pub note: Option<String>,
    pub accounting_quantity: f64,
    pub on_hand_quantity: f64,
    pub unit_of_measure: String,
    pub tracking_identifier: Option<String>,
    pub current_location: String,
    pub lot: Option<String>,
    pub contained_in: Option<ResourceSpecification>,
    pub resource_type: ResourceType,
    pub created_at: DateTime<Utc>,
    pub reference_number: i64
}

