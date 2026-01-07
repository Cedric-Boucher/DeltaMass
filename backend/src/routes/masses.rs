use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::PgPool;
use crate::{middleware::AuthSession, models::mass::{NewMass, Mass}, time_conversion::{convert_chrono_to_time, convert_time_to_chrono}};
use bigdecimal::{BigDecimal, ToPrimitive, FromPrimitive};
use futures::future::join_all;

pub fn routes() -> Router {
    Router::new().route("/masses", get(list_masses).post(create_mass))
        .route("/masses/{id}", get(get_mass).put(update_mass).delete(delete_mass))
}

async fn list_masses(
    Extension(pool): Extension<PgPool>,
    AuthSession(user): AuthSession,
) -> impl IntoResponse {
    let futures = sqlx::query!(
        r#"
        SELECT
            id as mass_id,
            mass_kg,
            masses.measurement_timestamp as mass_measurement_timestamp,
            masses.created_at as mass_created_at
        FROM masses
        WHERE masses.user_id = $1
        ORDER BY masses.measurement_timestamp DESC
        "#,
        user.id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch masses")
    .into_iter()
    .map(async |row| Mass {
        id: row.mass_id,
        mass_kg: row.mass_kg.to_f64().unwrap_or(0.0),
        measurement_timestamp: convert_time_to_chrono(row.mass_measurement_timestamp),
        created_at: convert_time_to_chrono(row.mass_created_at)
    });

    let rows: Vec<Mass> = join_all(futures).await;

    Json(rows)
}

pub async fn create_mass(
    Extension(pool): Extension<PgPool>,
    AuthSession(user): AuthSession,
    Json(payload): Json<NewMass>,
) -> Json<Mass> {
    let record = sqlx::query!(
        r#"
        INSERT INTO masses (user_id, mass_kg, measurement_timestamp)
        VALUES ($1, $2, COALESCE($3, now()))
        RETURNING id, mass_kg, measurement_timestamp, created_at
        "#,
        user.id,
        BigDecimal::from_f64(payload.mass_kg),
        payload.measurement_timestamp.map(convert_chrono_to_time),
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to insert mass");

    let result = Mass {
        id: record.id,
        mass_kg: record.mass_kg.to_f64().unwrap_or(0.0),
        measurement_timestamp: convert_time_to_chrono(record.measurement_timestamp),
        created_at: convert_time_to_chrono(record.created_at)
    };

    Json(result)
}

async fn get_mass(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    AuthSession(user): AuthSession
) -> impl IntoResponse {
    let existing = sqlx::query!(
        r#"
        SELECT id, mass_kg, measurement_timestamp, created_at
        FROM masses
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user.id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch mass");

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let row = existing.unwrap();

    let mass = Mass {
        id: row.id,
        mass_kg: row.mass_kg.to_f64().unwrap_or(0.0),
        measurement_timestamp: convert_time_to_chrono(row.measurement_timestamp),
        created_at: convert_time_to_chrono(row.created_at)
    };

    Ok(Json(mass))
}

async fn update_mass(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    AuthSession(user): AuthSession,
    Json(payload): Json<NewMass>,
) -> impl IntoResponse {
    let existing = sqlx::query!(
        r#"
        SELECT id FROM masses
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user.id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch mass");

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let row = sqlx::query!(
        r#"
        UPDATE masses
        SET
            mass_kg = $1,
            measurement_timestamp = COALESCE($2, measurement_timestamp)
        WHERE id = $3
        RETURNING id, mass_kg, measurement_timestamp, created_at
        "#,
        BigDecimal::from_f64(payload.mass_kg),
        payload.measurement_timestamp.map(convert_chrono_to_time),
        id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to update mass");

    let updated = Mass {
        id: row.id,
        mass_kg: row.mass_kg.to_f64().unwrap_or(0.0),
        measurement_timestamp: convert_time_to_chrono(row.measurement_timestamp),
        created_at: convert_time_to_chrono(row.created_at)
    };

    Ok(Json(updated))
}

async fn delete_mass(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    AuthSession(user): AuthSession
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        DELETE FROM masses
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user.id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() == 0 => Err(StatusCode::NOT_FOUND),
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
