use crate::endpoints::dns::Record;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateDnsRecord<'a> {
    zone_id: &'a str,
    dns_record_id: &'a str,
    params: UpdateDnsRecordParams,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDnsRecordParams {
    record: Record,
}

impl<'a> EndpointSpec<Record> for UpdateDnsRecord<'a> {
    fn method(&self) -> Method {
        Method::PATCH
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
