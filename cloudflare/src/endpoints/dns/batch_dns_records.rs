use crate::endpoints::dns::{BatchDnsRecords as BatchDnsRecordsRet, DnsRecord};
use crate::endpoints::Identifier;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BatchDnsRecords<'a> {
    zone_id: &'a str,
    params: BatchDnsRecordsParams,
}

#[derive(Serialize, Deserialize)]
pub struct BatchDnsRecordsParams {
    deletes: Option<Vec<Identifier>>,
    patches: Option<Vec<DnsRecord>>,
    posts: Option<Vec<DnsRecord>>,
    puts: Option<Vec<DnsRecord>>,
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
