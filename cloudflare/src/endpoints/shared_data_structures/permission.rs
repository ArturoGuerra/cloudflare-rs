use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(unused)]
type Permission = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionGrant {
    read: bool,
    write: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionGroup {
    id: String,
    meta: HashMap<String, String>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    analytics: Option<PermissionGrant>,
    billing: Option<PermissionGrant>,
    cache_purge: Option<PermissionGrant>,
    dns: Option<PermissionGrant>,
    dns_records: Option<PermissionGrant>,
    lb: Option<PermissionGrant>,
    logs: Option<PermissionGrant>,
    organization: Option<PermissionGrant>,
    ssl: Option<PermissionGrant>,
    waf: Option<PermissionGrant>,
    zone_settings: Option<PermissionGrant>,
    zones: Option<PermissionGrant>,
}
