use crate::endpoints::dns::DnsRecord;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OverwriteDnsRecord<'a> {
    zone_id: &'a str,
    dns_record_id: &'a str,
    params: OverwriteDnsRecordParams,
}

#[derive(Serialize, Deserialize)]
pub struct OverwriteDnsRecordParams {
    record: DnsRecord,
}

impl<'a> EndpointSpec<DnsRecord> for OverwriteDnsRecord<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!("zones/{}/dns_records/{}", self.zone_id, self.dns_record_id)
    }

    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}
