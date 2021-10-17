use fakeit::{contact, name, password};
use kuma::sdk::*;

#[tokio::test]
async fn test_user_registration() {
    // arrange
    let email = contact::email();
    let first = name::first();
    let last = name::last();
    let password = password::generate(true, true, false, 16);
    // create auth user
    let user = AuthUser::new(&email, &first, &last, &password).await;

    // registration
    let config = get_config().await;
    let flow = get_registration_flow(&config).await;
    let token = submit_registration_flow(&config, &flow, &user).await;

    // create login user
    let payload = VerificationRequest {
        method: "link".to_owned(),
        email: email,
    };

    let config = get_config().await;
    let flow = get_verification_flow(&config).await;
    let token = submit_verification_flow(&config, &flow, &token, &payload).await;

    assert!(token.len() > 0);
}
