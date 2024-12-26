use serde::{Deserialize, Serialize};

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
