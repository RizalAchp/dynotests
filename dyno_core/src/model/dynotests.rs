use crate::config::DynoConfig;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize)]
pub struct DynoTest {
    pub id: i64,
    pub user_id: i64,
    pub info_id: Option<i64>,
    pub uuid: String,
    pub data_url: String,
    pub data_checksum: String,
    pub verified: bool,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DynoTestDataInfo {
    pub checksum_hex: String,
    pub config: DynoConfig,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
}
