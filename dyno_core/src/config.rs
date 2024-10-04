#[cfg(feature = "std")]
use derive_more::Display;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use uom::si::{
    acceleration::meter_per_second_squared,
    f64::*,
    force::newton,
    length::{centimeter, meter},
    mass::kilogram,
    time::millisecond,
};

use crate::types::{MotorInfo, MotorKind, Stroke, GRAVITY_SPEED, PI};

#[cfg_attr(feature = "derive_serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "derive_serde", serde(default))]
#[cfg_attr(feature = "std", derive(Display))]
#[cfg_attr(feature = "std", display("{self:#?}"))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DynoConfig {
    pub diameter_roller_cm: f64,
    pub diameter_roller_beban_cm: f64,
    pub diameter_gear_encoder_cm: f64,
    pub diameter_gear_beban_cm: f64,
    pub berat_beban_kg: f64,

    pub motor_info: MotorInfo,
    pub max_encoder_pulse: u16,
    pub delta_ms: u16,
}

impl Default for DynoConfig {
    fn default() -> Self {
        Self {
            diameter_roller_cm: 14.22,
            diameter_roller_beban_cm: 19.33,
            berat_beban_kg: 18.5,
            diameter_gear_encoder_cm: 10.,
            diameter_gear_beban_cm: 5.4,
            max_encoder_pulse: 300,
            delta_ms: 250,
            motor_info: MotorInfo::default(),
        }
    }
}

impl DynoConfig {
    #[inline(always)]
    pub fn perbandingan_gear_encoder(&self) -> f64 {
        self.diameter_gear_encoder_cm / self.diameter_gear_beban_cm
    }
    #[inline(always)]
    pub fn perbandingan_gear_beban(&self) -> f64 {
        self.diameter_gear_beban_cm / self.diameter_gear_encoder_cm
    }
}

#[cfg(feature = "std")]
impl DynoConfig {
    #[inline(always)]
    pub fn radius_roller<UnitLength>(&self) -> f64
    where
        UnitLength: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        let radius_cm = Length::new::<centimeter>(self.diameter_roller_cm) * 0.5;
        radius_cm.get::<UnitLength>()
    }
    #[inline(always)]
    pub fn radius_roller_beban<UnitLength>(&self) -> f64
    where
        UnitLength: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        let radius_cm = Length::new::<centimeter>(self.diameter_roller_beban_cm) * 0.5;
        radius_cm.get::<UnitLength>()
    }

    #[inline(always)]
    pub fn circumference_roller<UnitLength>(&self) -> f64
    where
        UnitLength: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        let circumference_cm = Length::new::<centimeter>(self.diameter_roller_cm * PI);
        circumference_cm.get::<UnitLength>()
    }

    #[inline(always)]
    pub fn circumference_roller_beban<UnitLength>(&self) -> f64
    where
        UnitLength: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        let circumference_cm = Length::new::<centimeter>(self.diameter_roller_beban_cm * PI);
        circumference_cm.get::<UnitLength>()
    }

    #[inline(always)]
    pub fn rpm_factor(&self) -> f64 {
        if matches!(self.motor_info.kind, MotorKind::Electric) {
            return 1.;
        }
        match self.motor_info.stroke {
            Stroke::Four => 2. / ((self.motor_info.cylinder as u8) as f64),
            Stroke::Two => 1.,
        }
    }

    #[inline(always)]
    pub fn inertia_roller_beban(&self) -> f64 {
        let r = (self.diameter_roller_beban_cm * 0.5) / 100.;
        self.berat_beban_kg * r * r
    }

    #[inline(always)]
    pub fn force_roller_beban(&self) -> Force {
        let percepatan_gravitasi = Acceleration::new::<meter_per_second_squared>(GRAVITY_SPEED);
        Mass::new::<kilogram>(self.berat_beban_kg) * percepatan_gravitasi
    }

    #[inline(always)]
    pub fn torsi_roller_beban(&self) -> f64 {
        let f = self.force_roller_beban().get::<newton>();
        let r = self.radius_roller_beban::<meter>();
        f * r
    }

    pub fn delta_time(&self) -> Time {
        Time::new::<millisecond>(self.delta_ms as f64)
    }
}
