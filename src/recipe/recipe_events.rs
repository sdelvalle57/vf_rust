// Process Recipe Under Recipe(Process Map)
use chrono::{DateTime, Utc};

use crate::process::Process;
use crate::resource_specification::ResourceSpecification;
use crate::economic_resource::EconomicResource;

pub enum Action {
    PRODUCE,
    CONSUME,
    LOAD, 
    UNLOAD,
    TRANSFER,
    USE,
    CITE
}

pub enum Role {
    INPUT,
    OUTPUT
}

pub enum CommitmentStatus {
    PENDING,
    PARTIAL,
    FULFILLED
}

pub struct RecipeEvent {
    pub id: String,
    pub process: Process,
    pub action: Action,
    pub role: Role,
    pub resource_specification: Option<ResourceSpecification>,
    pub economic_resource: Option<EconomicResource>,
    pub note: String,
    pub is_commitment: Boolean, 
    // pub commitment_status: Option<CommitmentStatus>,
    pub triggers_commitment: Option<Boolean>,
    pub fulfills_commitment: RecipeEvent,
    pub location: String,
    pub created_at: DateTime<Utc>,

}

impl RecipeEvent {
    pub fn new(
        id: &str,
        process: Process,
        action: Action,
        role: Role,
        resource_specification: Option<ResourceSpecification>,
        economic_resource: Option<EconomicResource>,
        note: String,
        is_commitment: Boolean, 
        // commitment_status: Option<CommitmentStatus>,
        triggers_commitment: Option<Boolean>,
        fulfills_commitment: RecipeEvent,
        location: String
    ) -> RecipeEvent {
        RecipeEvent {
            id: id.to_string(),
            process,
            action,
            role,
            resource_specification,
            economic_resource,
            note: note.to_string(),
            is_commitment,
            triggers_commitment,
            fulfills_commitment,
            location,
            created_at: Utc::now()
        }
    }
}
