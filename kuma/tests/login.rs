use kuma::sdk::*;

#[tokio::test]
#[ignore]
async fn test_login() {
    // arrange
    let req = AuthRequest {
        method: "password".to_owned(),
        password_identifier: "steve@avengers.com".to_owned(),
        password: "jkhsdfg8934kln3469045".to_owned(),
    };

    let config = get_config().await;

    // login
    let flow = get_login_flow(&config).await;
    submit_login_flow(&config, flow, &req).await;

    assert!(true);
}
