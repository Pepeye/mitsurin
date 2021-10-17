use fakeit::{contact, name, password};
use kuma::sdk::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // arrange
    let email = contact::email();
    let first = name::first();
    let last = name::last();
    let password = password::generate(true, true, false, 16);
    // create auth user
    let user = AuthUser::new(&email, &first, &last, &password).await;

    // create login user
    let req = AuthRequest {
        method: "password".to_owned(),
        password_identifier: email,
        password,
    };

    // let vr = VerificationRequest {
    //     method: "link".to_owned(),
    //     email: email
    // };

    let config = get_config().await;
    let flow = get_registration_flow(&config).await;
    let token = submit_registration_flow(&config, &flow, &user).await;

    //verify email
    let _resp = verify_registration(&flow, &token);
    println!("[info]: verified user");

    // login
    let flow = get_login_flow(&config).await;
    submit_login_flow(&config, flow, &req).await;

    Ok(())
}
