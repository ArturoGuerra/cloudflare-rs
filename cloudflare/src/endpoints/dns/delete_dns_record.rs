use crate::endpoints::Identifier;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteDnsRecord<'a> {
    zone_id: &'a str,
    dns_record_id: &'a str,
    params: Identifier,
}

impl<'a> EndpointSpec<Identifier> for DeleteDnsRecord<'a> {
    fn method(&self) -> Method {
        Method::DELETE
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
