use crate::helpers::TestApp;
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::json;

#[tokio::test]
async fn test_unsubscribe_success() {
    let app = TestApp::new().await;

    // Insert test data
    sqlx::query!(
        r#"
        INSERT INTO swap_subscription (wallet_address, to_token)
        VALUES ($1, $2)
        ON CONFLICT (wallet_address) DO NOTHING
        "#,
        "0xdbfcab49bd9bced4636b04319d71fbd0d84bde78a1d38e9e2fc391e83187c1c3",
        "0x07ab8059db97aab8ced83b37a1d60b8eef540f6cdc96acc153d583a59bedd125"
    )
    .execute(&app.db.pool)
    .await
    .expect("Failed to insert test subscription");

    sqlx::query!(
        r#"
        INSERT INTO swap_subscription_from_token (wallet_address, from_token, swap_amount)
        VALUES ($1, $2, $3)
        ON CONFLICT (wallet_address, from_token) DO NOTHING
        "#,
        "0xdbfcab49bd9bced4636b04319d71fbd0d84bde78a1d38e9e2fc391e83187c1c3",
        "0x07ab8059db97aab8ced83b37a1d60b8eef540f6cdc96acc153d583a59bedd125",
        50
    )
    .execute(&app.db.pool)
    .await
    .expect("Failed to insert test from_token");

    let response = app
        .request(
            Request::builder()
                .method("POST")
                .uri("/unsubscribe")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "wallet_address": "0xdbfcab49bd9bced4636b04319d71fbd0d84bde78a1d38e9e2fc391e83187c1c3",
                        "from_token": "0x07ab8059db97aab8ced83b37a1d60b8eef540f6cdc96acc153d583a59bedd125"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024 * 16).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
}

#[tokio::test]
async fn test_unsubscribe_invalid_wallet() {
    let app = TestApp::new().await;

    let response = app
        .request(
            Request::builder()
                .method("POST")
                .uri("/unsubscribe")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "wallet_address": "invalid_wallet",
                        "from_token": "0x07ab8059db97aab8ced83b37a1d60b8eef540f6cdc96acc153d583a59bedd125"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_unsubscribe_invalid_token() {
    let app = TestApp::new().await;

    let response = app
        .request(
            Request::builder()
                .method("POST")
                .uri("/unsubscribe")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "wallet_address": "0x07ab8059db97aab8ced83b37a1d60b8eef540f6cdc96acc153d583a59bedd125",
                        "from_token": "invalid_token"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
