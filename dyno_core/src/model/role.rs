use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Deserialize,
    Serialize,
    Display,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(rename_all = "lowercase")]
pub enum Roles {
    #[display("admin")]
    Admin,
    #[display("user")]
    User,
    #[default]
    #[display("guest")]
    Guest,
}

impl Roles {
    #[allow(unused)]
    pub const fn is_admin(self) -> bool {
        matches!(self, Self::Admin)
    }
    #[allow(unused)]
    pub const fn is_user(self) -> bool {
        matches!(self, Self::Admin)
    }
    #[allow(unused)]
    pub const fn is_guest(self) -> bool {
        matches!(self, Self::Admin)
    }

    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Roles::Admin => "admin",
            Roles::User => "user",
            Roles::Guest => "guest",
        }
    }
}

impl<S: AsRef<str>> From<S> for Roles {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "admin" => Self::Admin,
            "user" => Self::User,
            "guest" => Self::Guest,
            _ => Self::Guest,
        }
    }
}

impl core::str::FromStr for Roles {
    type Err = String;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::User),
            "guest" => Ok(Self::Guest),
            _ => Err(format!(
                "Failed to parse from string in Roles, no role for {s}"
            )),
        }
    }
}
