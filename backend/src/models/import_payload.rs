use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ImportMasses {
    pub mass_kg: f64,
    pub measurement_timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ImportPayload {
    pub masses: Vec<ImportMasses>,
}
