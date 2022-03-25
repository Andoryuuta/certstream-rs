use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertstreamMessage {
    pub data: Data,
    #[serde(rename = "message_type")]
    pub message_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "cert_index")]
    pub cert_index: i64,
    #[serde(rename = "cert_link")]
    pub cert_link: String,
    #[serde(rename = "leaf_cert")]
    pub leaf_cert: LeafCert,
    pub seen: f64,
    pub source: Source,
    #[serde(rename = "update_type")]
    pub update_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeafCert {
    #[serde(rename = "all_domains")]
    pub all_domains: Vec<String>,
    pub extensions: Extensions,
    pub fingerprint: String,
    pub issuer: CertificateAttributes,
    #[serde(rename = "not_after")]
    pub not_after: i64,
    #[serde(rename = "not_before")]
    pub not_before: i64,
    #[serde(rename = "serial_number")]
    pub serial_number: String,
    #[serde(rename = "signature_algorithm")]
    pub signature_algorithm: String,
    pub subject: CertificateAttributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub authority_info_access: Option<String>,
    pub authority_key_identifier: Option<String>,
    pub basic_constraints: Option<String>,
    pub certificate_policies: Option<String>,
    pub ctl_signed_certificate_timestamp: Option<String>,
    pub extended_key_usage: Option<String>,
    pub key_usage: Option<String>,
    pub subject_alt_name: Option<String>,
    pub subject_key_identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateAttributes {
    #[serde(rename = "C")]
    pub country_name: Option<String>,
    #[serde(rename = "CN")]
    pub common_name: Option<String>,
    #[serde(rename = "L")]
    pub locality: Option<String>,
    #[serde(rename = "O")]
    pub organization: Option<String>,
    #[serde(rename = "OU")]
    pub organizational_unit: Option<String>,
    #[serde(rename = "ST")]
    pub state_or_provice_name: Option<String>,
    pub aggregated: Option<String>,
    pub email_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub url: String,
}
