use anyhow::Result;
use kratos::apis::{configuration::Configuration, v0alpha1_api as kratos_api};
use kratos::models::submit_self_service_verification_flow_with_link_method_body::SubmitSelfServiceVerificationFlowWithLinkMethodBody;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const KRATOS_ADMIN_ADDRESS: &'static str = "http://localhost:4433";
const KRATOS_VERIFICATION_ADDRESS: &'static str = "http://localhost:4455";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub method: String,
    pub password: String,
    #[serde(rename = "traits")]
    pub traits: PersonTraits,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonTraits {
    pub email: Option<String>,
    pub name: Option<Name>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub first: Option<String>,
    pub last: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub method: String,
    pub password_identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub method: String,
    pub email: String,
}

impl AuthUser {
    pub async fn new(email: &str, first: &str, last: &str, password: &str) -> Self {
        println!("[user]: email:{}, password: {}", &email, &password);

        // Initialise an auth user
        let au = AuthUser {
            method: "password".to_owned(),
            password: password.to_string(),
            traits: PersonTraits {
                email: Some(email.to_string()),
                name: Some(Name {
                    first: Some(first.to_string()),
                    last: Some(last.to_string()),
                }),
            },
        };

        au
    }
}

pub async fn get_config() -> Configuration {
    let mut config = Configuration::new();
    config.base_path = KRATOS_ADMIN_ADDRESS.to_owned();
    config
}

pub async fn get_registration_flow(config: &Configuration) -> String {
    // get registration flow
    let srf = kratos_api::initialize_self_service_registration_flow_without_browser(config)
        .await
        .unwrap();

    // get flow id
    let flow = &srf.id;
    println!("[registration|flow]: {:#?}", &flow);
    flow.to_string()
}

pub async fn submit_registration_flow(config: &Configuration, flow: &str, au: &AuthUser) -> String {
    // submit registration
    let body = Some(serde_json::to_value(au).unwrap());
    let resp = kratos_api::submit_self_service_registration_flow(config, flow, body)
        .await
        .unwrap();

    let token = &resp.session_token.unwrap();
    println!("[register|token]: {:#?}", &token);
    token.to_string()
}

pub async fn get_login_flow(config: &Configuration) -> String {
    // get registration flow
    let srf = kratos_api::initialize_self_service_registration_flow_without_browser(config)
        .await
        .unwrap();

    // get flow id
    let flow = &srf.id;
    println!("[login|flow]: {:#?}", &flow);
    flow.to_string()
}

pub async fn get_verification_flow(config: &Configuration) -> String {
    // get verification flow
    let srf = kratos_api::initialize_self_service_verification_flow_without_browser(config)
        .await
        .unwrap();

    // get flow id
    let flow = &srf.id;
    println!("[verification|flow]: {:#?}", &flow);
    flow.to_string()
}

pub async fn submit_verification_flow(
    config: &Configuration,
    flow: &str,
    token: &str,
    payload: &VerificationRequest
) -> String {
    let body = Some(serde_json::to_value(payload).unwrap());
    let resp = kratos_api::submit_self_service_verification_flow(config, &flow.to_owned(), Some(&token.to_owned()), body)
        .await
        .unwrap();

    println!("[verification|response]: {:#?}", &resp);
    // let token = &resp.session_token.unwrap();
    // println!("[login|token]: {:#?}", &token);
    "done".to_owned()
}

pub async fn verify_registration(flow: &str, token: &str) -> Result<String> {
    let url = format!(
        "{}/.ory/kratos/public/self-service/verification?flow={}&token={}",
        KRATOS_VERIFICATION_ADDRESS, flow, token
    );
    let res = Client::new().get(url).send().await?.text().await?;
    Ok(res)
}

pub async fn submit_login_flow(config: &Configuration, flow: String, req: &AuthRequest) -> String {
    // submit registration
    let body = Some(serde_json::to_value(req).unwrap());
    println!("{:#?}", &body);
    let resp = kratos_api::submit_self_service_login_flow(config, &flow, body)
        .await
        .unwrap();

    println!("{:#?}", &resp);
    let token = &resp.session_token.unwrap();
    println!("[login|token]: {:#?}", &token);
    token.to_string()
}
