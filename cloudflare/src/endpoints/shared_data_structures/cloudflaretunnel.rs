use std::net::IpAddr;

use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CloudflareTunnel {
    id: Uuid,
    account_tag: Option<String>,
    connections: Vec<CloudflareTunnelConnections>,
    conns_active_at: Option<DateTime<Utc>>,
    conns_inactive_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
    metadata: Option<serde_json::Value>,
    name: String,
    remote_config: bool,
    status: CloudflareTunnelStatus,
    tun_type: CloudflareTunnelTunType,
}

#[derive(Serialize, Deserialize)]
pub struct CloudflareTunnelConnections {
    id: Uuid,
    client_id: Uuid,
    client_version: String,
    color_name: String,
    is_pending_reconnect: bool,
    opened_at: DateTime<Utc>,
    origin_ip: IpAddr,
    uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub enum CloudflareTunnelStatus {
    Inactive,
    Degraded,
    Healthy,
    Down,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CloudflareTunnelTunType {
    CfdTunnel,
    WarpConnector,
    IpSec,
    Gre,
    Cni,
}
