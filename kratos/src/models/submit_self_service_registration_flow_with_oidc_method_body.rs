/*
 * Ory Kratos
 *
 * Welcome to the Ory Kratos HTTP API documentation!
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubmitSelfServiceRegistrationFlowWithOidcMethodBody : SubmitSelfServiceRegistrationFlowWithOidcMethodBody is used to decode the registration form payload when using the oidc method.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubmitSelfServiceRegistrationFlowWithOidcMethodBody {
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method to use  This field must be set to `oidc` when using the oidc method.
    #[serde(rename = "method")]
    pub method: String,
    /// The provider to register with
    #[serde(rename = "traits")]
    pub traits: String,
}

impl SubmitSelfServiceRegistrationFlowWithOidcMethodBody {
    /// SubmitSelfServiceRegistrationFlowWithOidcMethodBody is used to decode the registration form payload when using the oidc method.
    pub fn new(method: String, traits: String) -> SubmitSelfServiceRegistrationFlowWithOidcMethodBody {
        SubmitSelfServiceRegistrationFlowWithOidcMethodBody {
            csrf_token: None,
            method,
            traits,
        }
    }
}


