use std::net::{Ipv4Addr, Ipv6Addr};

use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ARecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Ipv4Addr>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DnsRecordTags>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct AAAARecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub content: Option<Ipv6Addr>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DnsRecordTags>>,
    pub ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CAARecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub data: CAARecordData,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DnsRecordTags>>,
    pub ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CAARecordData {
    pub flags: isize,
    pub tag: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CERTRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: CERTRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CERTRecordData {
    algorithm: isize,
    certificate: String,
    key_tag: isize,
    #[serde(rename = "type")]
    kind: isize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CNAMERecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub content: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DnsRecordTags>>,
    pub ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct DNSKEYRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: DNSKEYRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct DNSKEYRecordData {
    algorithm: usize,
    flags: usize,
    protocol: usize,
    public_key: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct DSRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: DSRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct DSRecordData {
    algorithm: usize,
    digest: String,
    digest_type: usize,
    key_tag: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct HTTPSRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: HTTPSRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct HTTPSRecordData {
    priority: Option<isize>,
    target: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct LOCRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: LOCRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct LOCRecordData {
    altitude: Option<isize>,
    lat_degress: Option<isize>,
    lat_direction: Option<LatDirection>,
    lat_minutes: Option<isize>,
    lat_seconds: Option<isize>,
    long_degrees: Option<isize>,
    long_direction: Option<LongDirection>,
    long_minutes: Option<isize>,
    long_seconds: Option<isize>,
    precision_horz: Option<isize>,
    precision_vert: Option<isize>,
    size: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum LatDirection {
    N,
    S,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum LongDirection {
    E,
    W,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MXRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    content: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct NAPTRRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: NAPTRRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct NAPTRRecordData {
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preference: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct NSRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    content: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct PTRRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    content: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SMIMEARecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: SMIMEARecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SMIMEARecordData {
    certificate: Option<String>,
    matching_type: Option<isize>,
    selector: Option<isize>,
    usage: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SRVRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: SRVRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SRVRecordData {
    port: Option<isize>,
    priority: Option<isize>,
    target: Option<String>,
    weight: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SSHFPRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: SSHFPRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SSHFPRecordData {
    algorithm: Option<isize>,
    fingerprint: Option<String>,
    #[serde(rename = "type")]
    kind: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SVCBRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: SVCBRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SVCBRecordData {
    priority: Option<isize>,
    target: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct TLSARecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: TLSARecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct TLSARecordData {
    certificate: Option<String>,
    matching_type: Option<isize>,
    selector: Option<isize>,
    usage: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct TXTRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    content: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct URIRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    data: URIRecordData,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct URIRecordData {
    target: Option<String>,
    weight: Option<isize>,
}
// Exclusive to Record
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct DNSRecordsOpengpgkeyRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    content: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<DnsRecordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DnsRecord {
    A(ARecord),
    AAAA(AAAARecord),
    CAA(CAARecord),
    CERT(CERTRecord),
    CNAME(CNAMERecord),
    DNSKEY(DNSKEYRecord),
    DS(DSRecord),
    HTTPS(HTTPSRecord),
    LOC(LOCRecord),
    MX(MXRecord),
    NAPTR(NAPTRRecord),
    NS(NSRecord),
    DNSRecordsOpengpgkey(DNSRecordsOpengpgkeyRecord), // Exclusive
    PTR(PTRRecord),
    SMIMEA(SMIMEARecord),
    SRV(SRVRecord),
    SSHFP(SSHFPRecord),
    SVCB(SVCBRecord),
    TLSA(TLSARecord),
    TXT(TXTRecord),
    URI(URIRecord),
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct DnsRecordSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten_cname: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

type DnsRecordTags = String;
type Ttl = usize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum DnsRecordKind {
    #[default]
    A,
    AAAA,
    CAA,
    CERT,
    CNAME,
    DNSKEY,
    DS,
    HTTPS,
    LOC,
    MX,
    NAPTR,
    NS,
    PTR,
    SMIMEA,
    SRV,
    SSHFP,
    SVCB,
    TLSA,
    TXT,
    URI,
    OPENPGPKEY,
}

// API Return types
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchDnsRecords {
    deletes: Option<Vec<DnsRecord>>,
    patches: Option<Vec<DnsRecord>>,
    posts: Option<Vec<DnsRecord>>,
    puts: Option<Vec<DnsRecord>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImportDnsRecords {
    recs_added: Option<String>,
    total_records_parsed: Option<String>,
}

impl ApiResult for String {}
impl ApiResult for DnsRecord {}
impl ApiResult for Vec<DnsRecord> {}
impl ApiResult for BatchDnsRecords {}
impl ApiResult for ImportDnsRecords {}
