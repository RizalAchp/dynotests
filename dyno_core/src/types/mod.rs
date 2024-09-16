#[cfg(feature = "std")]
mod data;
#[cfg(feature = "std")]
mod data_buffer;
#[cfg(feature = "std")]
mod infomotor;
mod serial;

#[cfg(feature = "std")]
pub type Float = f64;
#[cfg(feature = "std")]
pub const PI: Float = std::f64::consts::PI;
#[cfg(feature = "std")]
pub const GRAVITY_SPEED: Float = 9.806_65;

#[cfg(feature = "std")]
pub use data::*;
#[cfg(feature = "std")]
pub use data_buffer::*;
#[cfg(feature = "std")]
pub use infomotor::*;
pub use serial::*;
