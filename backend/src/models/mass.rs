use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Mass {
    pub id: i32,
    pub mass_kg: f64,
    pub measurement_timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewMass {
    pub mass_kg: f64,
    pub measurement_timestamp: Option<DateTime<Utc>>,
}
