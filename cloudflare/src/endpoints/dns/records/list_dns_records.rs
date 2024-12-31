use crate::endpoints::dns::records::DnsRecord;
use crate::endpoints::SortDirection;
use crate::framework::endpoint::{EndpointSpec, Method};
use serde::{Deserialize, Serialize};

use super::DnsRecordKind;

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecords<'a> {
    pub zone_id: &'a str,
    pub params: ListDnsReccordsParams,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ListDnsReccordsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<ListDnsRecordsParamsComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ListDnsRecordsParamsContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "match")]
    pub match_: Option<ListDnsRecordsParamsMatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ListDnsRecordsParamsName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListDnsRecordsParamsOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_match: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<DnsRecordKind>,
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParamsTag {
    pub absent: Option<String>,
    pub contains: Option<String>,
    pub endswith: Option<String>,
    pub exact: Option<String>,
    pub present: Option<String>,
    pub startswith: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ListDnsRecordsParamsTagMatch {
    Any,
    All,
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParamsName {
    pub contains: Option<String>,
    pub endswith: Option<String>,
    pub exact: Option<String>,
    pub startswith: Option<String>,
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
    pub absent: Option<String>,
    pub contains: Option<String>,
    pub endswitch: Option<String>,
    pub exact: Option<String>,
    pub present: Option<String>,
    pub startswith: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ListDnsRecordsParamsContent {
    pub contains: Option<String>,
    pub endswith: Option<String>,
    pub exact: Option<String>,
    pub startswith: Option<String>,
}

impl<'a> EndpointSpec<Vec<DnsRecord>> for ListDnsRecords<'a> {
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
