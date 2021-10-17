// use fakeit::{contact, name, password};
// use kuma::sdk::*;

// #[tokio::test]
// async fn test_user_registration() {
//     // arrange
//     let email = contact::email();
//     let first = name::first();
//     let last = name::last();
//     let password = password::generate(true, true, false, 16);
//     // create auth user
//     let user = AuthUser::new(&email, &first, &last, &password).await;

//     // create login user
//     let payload = AuthRequest {
//         method: "password".to_owned(),
//         password_identifier: email,
//         password,
//     };

//     let config = get_config().await;
//     let flow = get_verification_flow(&config).await;
//     let token = submit_verification_flow(&config, &flow, &user).await;

//     assert!(token.len() > 0);
// }
