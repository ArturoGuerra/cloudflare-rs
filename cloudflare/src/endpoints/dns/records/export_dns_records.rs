use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExportDnsRecords<'a> {
    pub zone_id: &'a str,
}

impl<'a> EndpointSpec<String> for ExportDnsRecords<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("zones/{}/dns_records/export", &self.zone_id)
    }

    #[inline]
    fn body(&self) -> Option<String> {
        None
    }
}
