use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuditLog {
    id: String,
    action: AuditLogAction,
    actor: AuditLogActor,
    interface: String,
    metdata: Option<serde_json::Value>,
    #[serde(rename = "newValue")]
    new_value: String,
    #[serde(rename = "oldValue")]
    old_value: String,
    owner: AuditLogOwner,
    resource: AuditLogResource,
    when: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuditLogAction {
    result: bool,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuditLogActor {
    id: String,
    email: String,
    ip: String,
    #[serde(rename = "type")]
    kind: AuditLogActorKind,
}

#[derive(Serialize, Deserialize)]
pub enum AuditLogActorKind {
    User,
    Admin,
    Cloudflare,
}

#[derive(Serialize, Deserialize)]
pub struct AuditLogOwner {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuditLogResource {
    id: String,
    #[serde(rename = "type")]
    kind: String,
}
