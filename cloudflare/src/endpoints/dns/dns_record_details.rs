use crate::endpoints::dns::DnsRecord;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DnsRecordDetails<'a> {
    zone_id: &'a str,
    dns_record_id: &'a str,
    params: DnsRecord,
}

impl<'a> EndpointSpec<DnsRecord> for DnsRecordDetails<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "zones/{}/dns_records/{}",
            &self.zone_id, &self.dns_record_id
        )
    }

    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}
