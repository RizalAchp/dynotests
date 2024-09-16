#[cfg(feature = "std")]
use derive_more::Display;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use core::prelude::rust_2021::derive;

#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "std",
    derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display)
)]
#[cfg_attr(feature = "std", display("{self:?}"))]
pub struct SerialDataInit {
    period_ms: u16,
}

#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "std",
    derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Display)
)]
#[cfg_attr(feature = "std", display("{self:?}"))]
pub struct RawSerialData {
    pub pulse_rpm: u16,
    pub pulse_enc: u16,
    pub temperature: u32,
}
impl RawSerialData {
    pub const fn new() -> Self {
        Self {
            pulse_rpm: 0,
            pulse_enc: 0,
            temperature: 0,
        }
    }
}
impl Default for RawSerialData {
    fn default() -> Self {
        Self::new()
    }
}

impl SerialDataInit {
    #[inline]
    pub fn period_ms(self) -> u16 {
        core::cmp::Ord::clamp(self.period_ms, 250, 2000)
    }
}

impl core::default::Default for SerialDataInit {
    fn default() -> Self {
        Self { period_ms: 250 }
    }
}
