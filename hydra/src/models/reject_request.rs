/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RejectRequest {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "error_debug", skip_serializing_if = "Option::is_none")]
    pub error_debug: Option<String>,
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "error_hint", skip_serializing_if = "Option::is_none")]
    pub error_hint: Option<String>,
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

impl RejectRequest {
    pub fn new() -> RejectRequest {
        RejectRequest {
            error: None,
            error_debug: None,
            error_description: None,
            error_hint: None,
            status_code: None,
        }
    }
}

