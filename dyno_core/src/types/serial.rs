#[cfg(feature = "std")]
use derive_more::Display;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

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
    pub raw_temp: u16,
}
impl RawSerialData {
    pub const BYTE_SIZE: usize = 8;
    pub const fn new() -> Self {
        Self {
            pulse_rpm: 0,
            pulse_enc: 0,
            raw_temp: 0,
        }
    }

    #[cfg(feature = "std")]
    pub fn temp_celcius(&self) -> Option<f64> {
        if (self.raw_temp & 0x4) != 0 {
            return None;
        }
        Some(((self.raw_temp >> 4) as f64) * 0.25_f64)
    }
}

impl SerialDataInit {
    #[inline]
    pub fn period_ms(self) -> u16 {
        core::cmp::Ord::clamp(self.period_ms, 250, 2000)
    }
}

impl core::default::Default for RawSerialData {
    fn default() -> Self {
        Self::new()
    }
}
impl core::default::Default for SerialDataInit {
    fn default() -> Self {
        Self { period_ms: 250 }
    }
}
