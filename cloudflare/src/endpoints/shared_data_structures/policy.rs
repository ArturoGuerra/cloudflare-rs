use super::PermissionGroup;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Policy {
    id: String,
    access: PolicyAccess,
    permission_groups: Vec<PermissionGroup>,
    resource_groups: Vec<PolicyResourceGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct PolicyResourceGroup {
    id: String,
    scope: Vec<PolicyResourceGroupScope>,
    meta: HashMap<String, String>,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PolicyResourceGroupScope {
    key: String,
    objects: Vec<PolicyResourceGroupScopeObjects>,
}

#[derive(Serialize, Deserialize)]
pub struct PolicyResourceGroupScopeObjects {
    key: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyAccess {
    Allow,
    Deny,
}
