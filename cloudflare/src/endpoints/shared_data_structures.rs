use std::net::IpAddr;

use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(unused)]
type Asn = isize;

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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificateCA {
    Digest,
    Google,
    LetsEncrypt,
    SslCom,
}

#[derive(Serialize, Deserialize)]
pub enum CertificateRequestType {
    #[serde(rename = "origin-rsa")]
    OriginRsa,
    #[serde(rename = "origin-ecc")]
    OriginEcc,
    #[serde(rename = "keyless-certificate")]
    KeylessCertificate,
}

#[derive(Serialize, Deserialize)]
pub struct CloudflareTunnel {
    id: Uuid,
    account_tag: String,
    connections: Vec<CloudflareTunnelConnections>,
    conns_active_at: Option<DateTime<Utc>>,
    conns_inactive_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
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

#[derive(Serialize, Deserialize)]
pub struct ErrorData {
    code: isize,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoadBalancerPreview {
    pools: Record,
    preview_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Member {
    id: isize,
    // TODO: add
    policies: Option<Vec<()>>,
    //TODO: ADD
    roles: Vec<Role>,
    status: Option<MemberStatus>,
    user: Option<User>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberPolicy {
    id: String,
    access: MemberPolicyAccess,
    permission_groups: Vec<MemberPolicyPermissionGroup>,
    resource_groups: Vec<MemberPolicyResourceGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberPolicyResourceGroup {
    id: String,
    scope: Vec<MemberPolicyResourceGroupScope>,
    meta: MemberPolicyResourceGroupMeta,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemberPolicyResourceGroupScope {
    key: String,
    objects: Vec<MemberPolicyResourceGroupScopeObjects>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberPolicyResourceGroupScopeObjects {
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemberPolicyResourceGroupMeta {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemberPolicyAccess {
    Allow,
    Deny,
}

#[derive(Serialize, Deserialize)]
pub struct MemberPolicyPermissionGroup {
    id: String,
    meta: MemberPolicyPermissionGroupMeta,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemberPolicyPermissionGroupMeta {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    email: String,
    id: String,
    first_name: String,
    last_name: String,
    two_factor_authentication_enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub enum MemberStatus {
    Accepted,
    Pending,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationInfo {
    count: isize,
    page: isize,
    per_page: isize,
    total_count: isize,
}

#[allow(unused)]
type Permission = String;

#[derive(Serialize, Deserialize)]
pub struct PermissionGrant {
    read: bool,
    write: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RatePlan {
    id: String,
    currency: String,
    externally_managed: bool,
    is_contract: bool,
    public_name: String,
    scope: String,
    sets: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseInfo {
    code: isize,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Role {
    id: String,
    description: String,
    name: String,
    permissions: Permissions,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize)]
pub struct Subscription {
    id: String,
    currency: String,
    current_period_end: String,
    current_period_start: String,
    frequency: SubscriptionFrequency,
    price: isize,
    rate_plan: RatePlan,
    state: SubscriptionState,
}

#[derive(Serialize, Deserialize)]
pub enum SubscriptionFrequency {
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Serialize, Deserialize)]
pub enum SubscriptionState {
    Trial,
    Provisioned,
    Paid,
    AwaitingPayment,
    Canceled,
    Failed,
    Expired,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionComponent {
    default: isize,
    name: String,
    price: isize,
    value: isize,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionZone {
    id: String,
    name: String,
}

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
    meta: TokenPermissionGroupMeta,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenPermissionGroupMeta {
    key: String,
    value: String,
}

#[allow(unused)]
type TokenValue = String;

#[derive(Serialize, Deserialize)]
pub struct Record(String, String);
