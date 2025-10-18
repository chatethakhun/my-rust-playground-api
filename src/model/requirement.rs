use serde::{Deserialize, Serialize};

use crate::model::{
    kit_part::{KitPart, KitPartRequirement},
    runner::Runner,
};

#[derive(Debug, Deserialize)]
pub struct BulkCreateRequirementsPayload {
    pub kit_part_id: i64,
    pub items: Vec<NewRequirementItem>,
}

#[derive(Debug, Deserialize)]
pub struct NewRequirementItem {
    pub gate: Vec<String>,
    pub qty: i32,
    pub is_cut: Option<bool>,
    pub runner_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateRequirementsPayload {
    pub items: Vec<UpdateRequirementItem>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequirementItem {
    pub id: i64,
    pub gate: Option<Vec<String>>,
    pub qty: Option<i32>,
    pub is_cut: Option<bool>,
    pub runner_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateKitPartRequirementPayload {
    pub gate: Vec<String>,
    pub qty: i32,
    pub runner_id: i64,
    pub kit_part_id: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct KitPartWithRequirements {
    #[serde(flatten)]
    pub kit_part: KitPart,
    pub requirements: Vec<KitPartRequirement>,
}

// #[derive(Debug, Deserialize)]
// pub struct UpdateKitPartRequirementPayload {
//     pub gate: Option<Vec<String>>,
//     pub qty: Option<i32>,
//     pub is_cut: Option<bool>,
//     pub runner_id: Option<i64>,
// }

#[derive(Debug, Deserialize)]
pub struct BulkSyncRequirementsPayload {
    pub kit_part_id: i64,
    pub create: Vec<NewRequirementItem>,
    pub update: Vec<UpdateRequirementItem>,
    pub delete_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CompareSyncRequirementsPayload {
    pub kit_part_id: i64,
    pub items: Vec<UpsertRequirementItem>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertRequirementItem {
    pub id: Option<i64>,
    pub gate: Vec<String>,
    pub qty: i32,
    pub is_cut: Option<bool>,
    pub runner_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequirementsPayload {
    pub ids: Vec<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct KitPartRequirementWithRunner {
    pub id: i64,
    pub gate: Vec<String>,
    pub qty: i64,
    pub is_cut: bool,
    pub runner_id: i64,
    pub kit_part_id: i64,
    pub user_id: i64,
    pub runner: Runner,
}
