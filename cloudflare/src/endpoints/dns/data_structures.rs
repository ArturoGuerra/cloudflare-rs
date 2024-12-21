use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ARecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be A
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct AAAARecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be AAAA
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct CAARecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<CAARecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be CAA
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct CAARecordData {
    flags: Option<isize>,
    tag: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CERTRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<CERTRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be CERT
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct CERTRecordData {
    algorithm: Option<isize>,
    certificate: Option<String>,
    key_tag: Option<isize>,
    // rename to type with serde
    kind: Option<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct CNAMERecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be CNAME
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct DNSKEYRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<DNSKEYRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be DNSKEY
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct DNSKEYRecordData {
    algorithm: Option<isize>,
    flags: Option<isize>,
    protocol: Option<isize>,
    public_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DSRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<DSRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be DS
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct DSRecordData {
    algorithm: Option<isize>,
    digest: Option<String>,
    digest_type: Option<isize>,
    key_tag: Option<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct HTTPSRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<HTTPSRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be HTTPS
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct HTTPSRecordData {
    priority: Option<isize>,
    target: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LOCRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<LOCRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be LOC
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum LatDirection {
    N,
    S,
}

#[derive(Serialize, Deserialize)]
pub enum LongDirection {
    E,
    W,
}

#[derive(Serialize, Deserialize)]
pub struct MXRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    priority: Option<isize>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be MX
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct NAPTRRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<NAPTRRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be NAPTR
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct NAPTRRecordData {
    flags: Option<String>,
    order: Option<isize>,
    preference: Option<isize>,
    regex: Option<String>,
    replacement: Option<String>,
    service: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NSRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be NS
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct PTRRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be PTR
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct SMIMEARecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SMIMEARecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be SMIMEA
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct SMIMEARecordData {
    certificate: Option<String>,
    matching_type: Option<isize>,
    selector: Option<isize>,
    usage: Option<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct SRVRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SRVRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be SRV
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct SRVRecordData {
    port: Option<isize>,
    priority: Option<isize>,
    target: Option<isize>,
    weight: Option<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct SSHFPRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SSHFPRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be SSHFP
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct SSHFPRecordData {
    algorithm: Option<isize>,
    fingerprint: Option<String>,
    kind: Option<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct SVCBRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<SVCBRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be SVCB
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct SVCBRecordData {
    priority: Option<isize>,
    target: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TLSARecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<TLSARecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be TLSA
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct TLSARecordData {
    certificate: Option<String>,
    matching_type: Option<isize>,
    selector: Option<isize>,
    usage: Option<isize>,
}

#[derive(Serialize, Deserialize)]
pub struct TXTRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be TXT
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct URIRecord {
    comment: Option<String>,
    content: Option<String>,
    data: Option<URIRecordData>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be URI
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
pub struct URIRecordData {
    target: Option<String>,
    weight: Option<isize>,
}

// Exclusive to Record
#[derive(Serialize, Deserialize)]
pub struct DNSRecordsOpengpgkeyRecord {
    comment: Option<String>,
    content: Option<String>,
    name: Option<String>,
    proxied: Option<bool>,
    settings: Option<RecordSettings>,
    tags: Option<Vec<RecordTags>>,
    ttl: Option<TTL>,
    // Should only be OPENPGPKEY
    kind: RecordKind,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct RecordSettings {
    ipv4_only: Option<bool>,
    ipv6_only: Option<bool>,
}

type RecordTags = String;
type TTL = isize;

#[derive(Serialize, Deserialize)]
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
