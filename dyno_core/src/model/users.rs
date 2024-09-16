use super::role::Roles;
use chrono::NaiveDateTime;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Display, Debug, Clone, PartialEq)]
#[display("{self:#?}")]
pub struct UserResponse {
    pub id: i64,
    pub uuid: String,
    pub nim: String,
    pub name: String,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub role: Roles,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Deserialize, Serialize, Display, Debug, Default, Clone, PartialEq)]
#[display("{self:#?}")]
pub struct UserRegistration {
    pub nim: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub role: Roles,
}

#[derive(Deserialize, Serialize, Display, Debug, Default, Clone)]
#[display("{self:#?}")]
pub struct UserLogin {
    pub nim: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Display, Debug, Default, Clone)]
#[display("{self:#?}")]
pub struct UserUpdate {
    pub nim: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role: Option<Roles>,
    pub email: Option<String>,
    pub photo: Option<String>,
}
