#[cfg(feature = "std")]
use derive_more::Display;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[cfg_attr(feature = "std", derive(Display))]
#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stroke {
    Two = 2,

    #[default]
    Four = 4,
}

#[repr(u8)]
#[cfg_attr(feature = "std", derive(Display))]
#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Transmition {
    Unknown,
    #[default]
    Automatic,
    Manual,
}

#[repr(u8)]
#[cfg_attr(feature = "std", derive(Display))]
#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cylinder {
    #[default]
    Single = 1,
    Two,
    Triple,
    Four,
    Six,
    Eight,
}

#[repr(u8)]
#[cfg_attr(feature = "std", derive(Display))]
#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MotorKind {
    #[default]
    Engine,
    Electric,
}

#[cfg_attr(feature = "std", derive(Display), display("{self:#?}"))]
#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "derive_serde", serde(default))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MotorInfo {
    pub cc: u16,
    pub kind: MotorKind,
    pub cylinder: Cylinder,
    pub stroke: Stroke,
    pub transmition: Transmition,
}

impl Stroke {
    pub fn into_iter() -> impl Iterator<Item = Self> {
        [Self::Two, Self::Four].into_iter()
    }
}

impl From<u8> for Stroke {
    #[inline(always)]
    fn from(val: u8) -> Self {
        match val {
            2 => Self::Two,
            4 => Self::Four,
            _ => Self::default(),
        }
    }
}

impl Transmition {
    pub fn into_iter() -> impl Iterator<Item = Self> {
        [Self::Unknown, Self::Automatic, Self::Manual].into_iter()
    }
}

impl From<u8> for Transmition {
    #[inline(always)]
    fn from(val: u8) -> Self {
        match val {
            0 => Transmition::Automatic,
            1..=8 => Transmition::Manual,
            _ => Transmition::Unknown,
        }
    }
}

impl Cylinder {
    pub fn into_iter() -> impl Iterator<Item = Self> {
        [
            Self::Single,
            Self::Two,
            Self::Triple,
            Self::Four,
            Self::Six,
            Self::Eight,
        ]
        .into_iter()
    }
}

impl From<u8> for Cylinder {
    fn from(val: u8) -> Self {
        match val {
            1 => Cylinder::Single,
            2 => Cylinder::Two,
            3 => Cylinder::Triple,
            4 => Cylinder::Four,
            6 => Cylinder::Six,
            8 => Cylinder::Eight,
            _ => Cylinder::default(),
        }
    }
}

impl MotorKind {
    pub fn is_electric(&self) -> bool {
        matches!(self, Self::Electric)
    }

    pub fn is_engine(&self) -> bool {
        matches!(self, Self::Engine)
    }
}

impl From<u8> for MotorKind {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Engine,
            1 => Self::Electric,
            _ => Self::Engine,
        }
    }
}

impl Default for MotorInfo {
    fn default() -> Self {
        Self {
            cc: 125,
            cylinder: Cylinder::Single,
            stroke: Stroke::Four,
            transmition: Transmition::Manual,
            kind: MotorKind::Engine,
        }
    }
}

impl MotorInfo {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn set_cc(&mut self, cc: impl Into<u16>) -> &mut Self {
        self.cc = cc.into();
        self
    }

    #[inline(always)]
    pub fn set_cylinder(&mut self, cylinder: impl Into<Cylinder>) -> &mut Self {
        self.cylinder = cylinder.into();
        self
    }

    #[inline(always)]
    pub fn set_stroke(&mut self, stroke: impl Into<Stroke>) -> &mut Self {
        self.stroke = stroke.into();
        self
    }

    #[inline(always)]
    pub fn set_transmition(&mut self, transmition: impl Into<Transmition>) -> &mut Self {
        self.transmition = transmition.into();
        self
    }
}
