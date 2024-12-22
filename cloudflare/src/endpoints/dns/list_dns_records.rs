use crate::endpoints::dns::Record;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecords<'a> {
    zone_id: &'a str,
    params: ListDnsRecordsParams,
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParams {
    comment: Option<ListDnsRecordsParamsComment>,
    content: Option<ListDnsRecordsParamsContent>,
    direction: String,
    r#match: String,
    name: String,
    order: String,
    page: Option<isize>,
    per_page: Option<isize>,
    proxied: Option<isize>,
    search: Option<String>,
    tag: String,
    tag_match: String,
    // TODO: Rename to type
    kind: String,
}

impl<'a> EndpointSpec<Vec<Record>> for ListDnsRecords<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("zones/{}/dns_records", &self.zone_id)
    }

    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParamsComment {
    absent: Option<String>,
    contains: Option<String>,
    endswitch: Option<String>,
    exact: Option<String>,
    present: Option<String>,
    startswith: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParamsContent {
    contains: Option<String>,
    endswith: Option<String>,
    exact: Option<String>,
    startswith: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ListDnsRecordsParamsDirection {}
