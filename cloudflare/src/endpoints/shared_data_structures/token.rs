use super::Record;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Token {
    id: String,
    condition: TokenCondition,
    expires_on: String,
    issued_on: String,
    last_used_on: String,
    modified_on: String,
    name: String,
    not_before: String,
    policies: Vec<TokenPolicy>,
    status: TokenStatus,
}

#[derive(Serialize, Deserialize)]
pub struct TokenCondition {
    request_ip: TokenCondtionRequestIP,
}

#[derive(Serialize, Deserialize)]
pub struct TokenCondtionRequestIP {
    #[serde(rename = "in")]
    r#in: Vec<TokenCondtionCIDRList>,
    not_in: Vec<TokenCondtionCIDRList>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenStatus {
    Active,
    Disabled,
    Expired,
}

type TokenCondtionCIDRList = String;

#[derive(Serialize, Deserialize)]
pub struct TokenPolicy {
    id: String,
    effect: TokenPolicyEffect,
    permission_groups: Vec<TokenPermissionGroup>,
    resources: Record,
}

#[derive(Serialize, Deserialize)]
pub enum TokenPolicyEffect {
    Allow,
    Deny,
}

#[derive(Serialize, Deserialize)]
pub struct TokenPermissionGroup {
    id: String,
    meta: HashMap<String, String>,
    name: String,
}

#[allow(unused)]
type TokenValue = String;
