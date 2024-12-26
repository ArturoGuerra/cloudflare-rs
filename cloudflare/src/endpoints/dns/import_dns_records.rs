use crate::endpoints::dns::ImportDnsRecords as ImportDnsRecordsRet;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ImportDnsRecords<'a> {
    zone_id: &'a str,
    params: ImportDnsRecordsParams,
}

#[derive(Serialize, Deserialize)]
pub struct ImportDnsRecordsParams {
    // TODO: Fingure out how to do this
    file: String,
    proxied: Option<String>,
}

impl<'a> EndpointSpec<ImportDnsRecordsRet> for ImportDnsRecords<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("zones/{}/dns_records/import", &self.zone_id)
    }

    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}
