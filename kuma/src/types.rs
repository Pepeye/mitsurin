use serde::{Deserialize, Serialize};
use fakeit::{contact, name, password};

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