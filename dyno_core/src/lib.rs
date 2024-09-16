#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_comparisons)]

#[cfg(feature = "std")]
pub mod config;
pub mod types;

#[cfg(feature = "use_model")]
pub mod model;

#[macro_export]
macro_rules! ternary {
    (($logic:expr) ? ($trues:expr) : ($falsies:expr)) => {
        if $logic {
            $trues
        } else {
            $falsies
        }
    };
}

#[cfg(feature = "std")]
pub use derive_more;
#[cfg(feature = "std")]
pub use log;
#[cfg(feature = "derive_serde")]
pub use serde;
#[cfg(feature = "std")]
pub use uom;
