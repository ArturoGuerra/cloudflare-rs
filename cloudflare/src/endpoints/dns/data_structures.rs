use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ARecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be A
    kind: RecordKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AAAARecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be AAAA
    kind: RecordKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAARecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<CAARecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be CAA
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CAARecordData {
    flags: Option<isize>,
    tag: Option<String>,
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CERTRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<CERTRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be CERT
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CERTRecordData {
    algorithm: Option<isize>,
    certificate: Option<String>,
    key_tag: Option<isize>,
    // rename to type with serde
    kind: Option<isize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CNAMERecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be CNAME
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DNSKEYRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<DNSKEYRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be DNSKEY
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DNSKEYRecordData {
    algorithm: Option<isize>,
    flags: Option<isize>,
    protocol: Option<isize>,
    public_key: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DSRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<DSRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be DS
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DSRecordData {
    algorithm: Option<isize>,
    digest: Option<String>,
    digest_type: Option<isize>,
    key_tag: Option<isize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPSRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<HTTPSRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be HTTPS
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPSRecordData {
    priority: Option<isize>,
    target: Option<String>,
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LOCRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<LOCRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be LOC
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub enum LatDirection {
    N,
    S,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum LongDirection {
    E,
    W,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MXRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    priority: Option<isize>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be MX
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NAPTRRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<NAPTRRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be NAPTR
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NAPTRRecordData {
    flags: Option<String>,
    order: Option<isize>,
    preference: Option<isize>,
    regex: Option<String>,
    replacement: Option<String>,
    service: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NSRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be NS
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PTRRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be PTR
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SMIMEARecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SMIMEARecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be SMIMEA
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SMIMEARecordData {
    certificate: Option<String>,
    matching_type: Option<isize>,
    selector: Option<isize>,
    usage: Option<isize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SRVRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SRVRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be SRV
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SRVRecordData {
    port: Option<isize>,
    priority: Option<isize>,
    target: Option<isize>,
    weight: Option<isize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SSHFPRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SSHFPRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be SSHFP
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SSHFPRecordData {
    algorithm: Option<isize>,
    fingerprint: Option<String>,
    kind: Option<isize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SVCBRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SVCBRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be SVCB
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SVCBRecordData {
    priority: Option<isize>,
    target: Option<String>,
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TLSARecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<TLSARecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be TLSA
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TLSARecordData {
    certificate: Option<String>,
    matching_type: Option<isize>,
    selector: Option<isize>,
    usage: Option<isize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TXTRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be TXT
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct URIRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<URIRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be URI
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct URIRecordData {
    target: Option<String>,
    weight: Option<isize>,
}
// Exclusive to Record
#[derive(Serialize, Deserialize, Debug)]
pub struct DNSRecordsOpengpgkeyRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<Ttl>,
    // Should only be OPENPGPKEY
    kind: RecordKind,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Record {
    ARecord(ARecord),
    AAAARecord(AAAARecord),
    CAARecord(CAARecord),
    CERTRecord(CERTRecord),
    CNAMERecord(CNAMERecord),
    DNSKEYRecord(DNSKEYRecord),
    DSRecord(DSRecord),
    HTTPSRecord(HTTPSRecord),
    LOCRecord(LOCRecord),
    MXRecord(MXRecord),
    NAPTRecord(NAPTRRecord),
    NSRecord(NSRecord),
    DNSRecordsOpengpgkeyRecord(DNSRecordsOpengpgkeyRecord), // Exclusive
    PTRRecord(PTRRecord),
    SMIMEARecord(SMIMEARecord),
    SRVRecord(SRVRecord),
    SSHFPRecord(SSHFPRecord),
    SVCBRecord(SVCBRecord),
    TLSARecord(TLSARecord),
    TXTRecord(TXTRecord),
    URIRecord(URIRecord),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RecordSettings {
    ipv4_only: Option<bool>,
    ipv6_only: Option<bool>,
}

type RecordTags = String;
type Ttl = isize;

#[derive(Serialize, Deserialize, Debug)]
pub enum RecordKind {
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
    deletes: Option<Vec<Record>>,
    patches: Option<Vec<Record>>,
    posts: Option<Vec<Record>>,
    puts: Option<Vec<Record>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImportDnsRecords {
    recs_added: Option<String>,
    total_records_parsed: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteId {
    id: String,
}

impl ApiResult for String {}
impl ApiResult for Record {}
impl ApiResult for DeleteId {}
impl ApiResult for Vec<Record> {}
impl ApiResult for BatchDnsRecords {}
impl ApiResult for ImportDnsRecords {}
