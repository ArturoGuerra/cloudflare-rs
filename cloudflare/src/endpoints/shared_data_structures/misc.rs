use crate::framework::response::ApiResult;

use super::Permissions;
use serde::{Deserialize, Serialize};

#[allow(unused)]
type Asn = isize;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorData {
    code: isize,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadBalancerPreview {
    pools: Record,
    preview_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationInfo {
    count: isize,
    page: isize,
    per_page: isize,
    total_count: isize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RatePlan {
    id: String,
    currency: String,
    externally_managed: bool,
    is_contract: bool,
    public_name: String,
    scope: String,
    sets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseInfo {
    code: isize,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    id: String,
    description: String,
    name: String,
    permissions: Permissions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record(String, String);

impl ApiResult for Identifier {}
