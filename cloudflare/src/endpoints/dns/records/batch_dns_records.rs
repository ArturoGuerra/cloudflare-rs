use crate::endpoints::dns::records::{BatchDnsRecords as BatchDnsRecordsRet, DnsRecord};
use crate::endpoints::Identifier;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BatchDnsRecords<'a> {
    pub zone_id: &'a str,
    pub params: BatchDnsRecordsParams,
}

#[derive(Serialize, Deserialize)]
pub struct BatchDnsRecordsParams {
    pub deletes: Option<Vec<Identifier>>,
    pub patches: Option<Vec<DnsRecord>>,
    pub posts: Option<Vec<DnsRecord>>,
    pub puts: Option<Vec<DnsRecord>>,
}

impl<'a> EndpointSpec<BatchDnsRecordsRet> for BatchDnsRecords<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("zones/{}/dns_records/batch", &self.zone_id)
    }

    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}
