use crate::endpoints::dns::Record;
use crate::endpoints::SortDirection;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

use super::RecordKind;

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecords<'a> {
    zone_id: &'a str,
    params: ListDnsReccordsParams,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ListDnsReccordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<ListDnsRecordsParamsComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<ListDnsRecordsParamsContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<SortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "match")]
    r#match: Option<ListDnsRecordsParamsMatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListDnsRecordsParamsName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<ListDnsRecordsParamsOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_match: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    kind: Option<RecordKind>,
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParamsTag {
    absent: Option<String>,
    contains: Option<String>,
    endswith: Option<String>,
    exact: Option<String>,
    present: Option<String>,
    startswith: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ListDnsRecordsParamsTagMatch {
    Any,
    All,
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParamsName {
    contains: Option<String>,
    endswith: Option<String>,
    exact: Option<String>,
    startswith: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ListDnsRecordsParamsOrder {
    Type,
    Name,
    Content,
    Ttl,
    Proxied,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListDnsRecordsParamsMatch {
    Any,
    All,
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
