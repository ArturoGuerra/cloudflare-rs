use crate::framework::endpoint::{EndpointSpec, Method};

#[derive(Debug)]
pub struct BatchDnsRecords<'a> {
    zone_id: &'a str,
    params: BatchDnsRecordsParams,
}

#[derive(Debug)]
pub struct BatchDnsRecordsParams {}

impl<'a> EndpointSpec<()> for BatchDnsRecords<'a> {
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
