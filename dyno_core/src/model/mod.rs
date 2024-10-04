use chrono::{DateTime, Utc};

use crate::config::DynoConfig;

pub mod dynotests;
pub mod role;
pub mod users;

pub const COOKIE_NAME: &str = "dyno_session";
pub const USER_HEADER_NAME: &str = "x-user-id";
pub const APP_USER_AGENT: &str = "dynotests/desktop-app";

#[cfg(feature = "std")]
use derive_more::Display;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Display)]
#[display("{self:#?}")]
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UserSession {
    pub id: i64,
    pub uuid: String,
    pub role: role::Roles,
}

#[derive(Deserialize, Serialize, Display)]
#[display("{self:#?}")]
#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub id: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

impl TokenClaims {
    #[allow(unused)]
    pub fn new(id: impl ToString, max_age: i64, sub: impl ToString) -> Self {
        let now = chrono::Utc::now();
        let iat = now.timestamp_millis();
        let nbf = now.timestamp_millis();
        let exp = (now + chrono::Duration::minutes(max_age as _)).timestamp_millis();
        Self {
            id: id.to_string(),
            sub: sub.to_string(),
            exp,
            iat,
            nbf,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ActiveResponse<'s> {
    pub user: Option<users::UserResponse<'s>>,
    pub dyno: Option<DynoConfig>,
    pub start: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Display)]
#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResponseStatus {
    #[display("success")]
    Success,
    #[display("error")]
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub id: i64,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ApiResponse<T> {
    pub payload: T,
    pub status: ResponseStatus,
}

impl<T> ApiResponse<T>
where
    T: serde::ser::Serialize,
    T: serde::de::DeserializeOwned,
{
    pub fn success(payload: T) -> Self {
        Self {
            payload,
            status: ResponseStatus::Success,
        }
    }

    pub fn error(payload: T) -> Self {
        Self {
            payload,
            status: ResponseStatus::Error,
        }
    }

    pub const fn status_ok(&self) -> bool {
        matches!(self.status, ResponseStatus::Success)
    }
}
