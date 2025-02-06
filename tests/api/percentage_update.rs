use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;

use crate::helpers::*;

async fn setup_test_data(pool: &PgPool, wallet_address: &str, from_token: &str, swap_amount: i64) {
    sqlx::query!(
        r#"
        INSERT INTO swap_subscription (wallet_address, to_token, is_active)
        VALUES ($1, $2, true)
        ON CONFLICT (wallet_address)
        DO UPDATE SET to_token = $2, is_active = true, updated_at = NOW()
        "#,
        wallet_address,
        "0x07ab8059db97aab8ced83b37a1d60b8eef540f6cdc96acc153d583a59bedd125"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
            INSERT INTO swap_subscription_from_token
            (wallet_address, from_token, swap_amount)
            VALUES ($1, $2, $3)
            "#,
        wallet_address,
        from_token,
        swap_amount,
    )
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn test_update_percentage_success() {
    let app = TestApp::new().await;

    clean_database(&app.db.pool).await;

    let wallet_address = "0xed0fd074f3acf231815432ad61dcce077a488fbd05a27e37471de432a32c1656";
    let from_token = "0x1dcb0e5a46ae5a8f49fd948ebb0dcbc96d909ea35c5b312bc719bff47cb8720f";
    let initial_percentage = 50;

    setup_test_data(&app.db.pool, wallet_address, from_token, initial_percentage).await;

    let payload = json!({
        "wallet_address": wallet_address,
        "from_token": from_token,
        "swap_amount": 75
    });

    let req = Request::builder()
        .method("PATCH")
        .uri("/update_percentage")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let updated = sqlx::query!(
        r#"
        SELECT swap_amount
        FROM swap_subscription_from_token
        WHERE wallet_address = $1 AND from_token = $2
        "#,
        wallet_address,
        from_token
    )
    .fetch_one(&app.db.pool)
    .await
    .unwrap();

    assert_eq!(updated.swap_amount, 75);
}

#[tokio::test]
async fn test_update_percentage_not_found() {
    let app = TestApp::new().await;

    clean_database(&app.db.pool).await;

    let payload = json!({
        "wallet_address": "0x1dcb0e5a46ae5a8f49fd948ebb0dcbc96d909ea35c5b312bc719bff47cb8720f",
        "from_token": "0xed0fd074f3acf231815432ad61dcce077a488fbd05a27e37471de432a32c1656",
        "swap_amount": 75
    });

    let req = Request::builder()
        .method("PATCH")
        .uri("/update_percentage")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let resp = app.request(req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
