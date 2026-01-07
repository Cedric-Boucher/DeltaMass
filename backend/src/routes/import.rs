use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use crate::{
    middleware::AuthSession,
    models::import_payload::ImportPayload,
    time_conversion::convert_chrono_to_time
};
use sqlx::PgPool;
use bigdecimal::{BigDecimal, FromPrimitive};

pub fn routes() -> Router {
    Router::new().route("/import", post(import_data))
}

pub async fn import_data(
    Extension(pool): Extension<PgPool>,
    AuthSession(user): AuthSession,
    Json(payload): Json<ImportPayload>,
) -> StatusCode {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    for tx_item in payload.masses {
        let mass_kg = BigDecimal::from_f64(tx_item.mass_kg).unwrap_or(BigDecimal::from(0));

        let _ = sqlx::query!(
            r#"
            INSERT INTO masses (user_id, mass_kg, measurement_timestamp, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
            user.id,
            mass_kg,
            convert_chrono_to_time(tx_item.measurement_timestamp),
            convert_chrono_to_time(tx_item.created_at),
        )
        .execute(&mut *tx)
        .await
        .expect("Failed to insert mass");
    }

    // Commit everything
    if let Err(_) = tx.commit().await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
