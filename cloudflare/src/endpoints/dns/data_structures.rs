use std::net::{Ipv4Addr, Ipv6Addr};

use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ARecord {
    comment: Option<String>,
    content: Ipv4Addr,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AAAARecord {
    comment: Option<String>,
    content: Ipv6Addr,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CAARecord {
    comment: Option<String>,
    content: Option<String>,
    data: CAARecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CAARecordData {
    flags: isize,
    tag: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CERTRecord {
    comment: Option<String>,
    content: Option<String>,
    data: CERTRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: String,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct DNSKEYRecord {
    comment: Option<String>,
    content: Option<String>,
    data: DNSKEYRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: Option<String>,
    data: DSRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: Option<String>,
    data: HTTPSRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: Option<String>,
    data: LOCRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: String,
    name: String,
    priority: Option<isize>,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct NAPTRRecord {
    comment: Option<String>,
    content: Option<String>,
    data: NAPTRRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct NAPTRRecordData {
    flags: Option<String>,
    order: Option<isize>,
    preference: Option<isize>,
    regex: Option<String>,
    replacement: Option<String>,
    service: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct NSRecord {
    comment: Option<String>,
    content: String,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct PTRRecord {
    comment: Option<String>,
    content: String,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SMIMEARecord {
    comment: Option<String>,
    content: Option<String>,
    data: SMIMEARecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: Option<String>,
    data: SRVRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: Option<String>,
    data: SSHFPRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: Option<String>,
    data: SVCBRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: Option<String>,
    data: TLSARecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: String,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct URIRecord {
    comment: Option<String>,
    content: Option<String>,
    data: URIRecordData,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    comment: Option<String>,
    content: String,
    name: String,
    proxied: Option<bool>,
    settings: Option<DnsRecordSettings>,
    tags: Option<Vec<DnsRecordTags>>,
    ttl: Ttl,
    zone_id: Option<String>,
    zone_name: Option<String>,
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
    NAPT(NAPTRRecord),
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
    flatten_cname: Option<bool>,
    ipv4_only: Option<bool>,
    ipv6_only: Option<bool>,
}

type DnsRecordTags = String;
type Ttl = usize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DnsRecordKind {
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
