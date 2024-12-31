use crate::endpoints::dns::records::DnsRecord;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OverwriteDnsRecord<'a> {
    pub zone_id: &'a str,
    pub dns_record_id: &'a str,
    pub params: OverwriteDnsRecordParams,
}

#[derive(Serialize, Deserialize)]
pub struct OverwriteDnsRecordParams {
    pub record: DnsRecord,
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
