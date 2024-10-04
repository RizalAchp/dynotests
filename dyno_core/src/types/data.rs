use super::{DynotestData, RawSerialData};

use derive_more::Display;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

use chrono::{DateTime, TimeDelta, Utc};
use si_type::*;
use uom::ConstZero;

#[cfg_attr(
    feature = "derive_serde",
    derive(Deserialize, Serialize),
    serde(default)
)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Display)]
#[display("{self:#?}")]
pub struct Data {
    /// unit [KMpH]
    pub speed: Velocity,
    /// unit [NM]
    pub torque: Torque,
    /// unit [HP]
    pub horsepower: Power,
    /// unit [REVpM]
    pub rpm_roda: AngularVelocity,
    /// unit [REVpM]
    pub rpm_engine: AngularVelocity,
    /// unit [Celcius]
    pub temp: TemperatureInterval,

    pub timestamp: i64,
}
impl Data {
    pub const ZERO: Data = Data {
        speed: ConstZero::ZERO,
        torque: ConstZero::ZERO,
        horsepower: ConstZero::ZERO,
        rpm_roda: ConstZero::ZERO,
        rpm_engine: ConstZero::ZERO,
        temp: ConstZero::ZERO,
        timestamp: DateTime::UNIX_EPOCH.timestamp_millis(),
    };

    const SIZE_IDX: usize = 7;
    pub const BUFFER_NAME: [&'static str; Self::SIZE_IDX] = [
        "SPEED",
        "RPM Roda",
        "RPM Engine",
        "TORQUE",
        "HORSEPOWER",
        "TEMPERATURE",
        "TIMESTAMP",
    ];

    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn timestamp_format(&self, fmt: &str) -> String {
        self.timestamp_date_time().format(fmt).to_string()
    }

    #[inline]
    pub fn timestamp_date_time(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(self.timestamp).unwrap_or_default()
    }
}

pub(crate) fn calculate_odo_from_data(last_time: &DateTime<Utc>, data: &Data) -> Length {
    let time = data.timestamp_date_time() - last_time;
    let sec = time.num_seconds() as f64;
    let sub_sec = time.subsec_nanos() as f64 / (0x3B9ACA00 as f64);
    let sec = Time::new::<second>(sec + sub_sec);
    Length::new::<meter>(data.speed.value * sec.get::<hour>())
}
pub(crate) fn calculate_odo_from_datas(last_time: DateTime<Utc>, data: &[Data]) -> Length {
    data.iter().fold(Length::ZERO, |odo, d| {
        odo + calculate_odo_from_data(&last_time, d)
    })
}

impl DynotestData {
    #[inline(always)]
    pub fn push_from_raw_serial_data(
        &mut self,
        raw @ RawSerialData {
            pulse_rpm,
            pulse_enc,
            ..
        }: RawSerialData,
    ) {
        let last_data = self.last();

        let delta_time = self.config.delta_time();

        let timestamp = {
            let delta_ms = TimeDelta::milliseconds(self.config.delta_ms as _);
            self.start_time
                .checked_add_signed(delta_ms)
                .map(|x| x.timestamp_millis())
                .unwrap_or(last_data.timestamp)
        };

        // change from phase A to phase Z (pulse per revolution)
        // let roller_revolution = pulse_enc / f64::from(self.config.max_encoder_pulse);
        let roller_revolution = pulse_enc as f64;
        let engine_revolution = (pulse_rpm as f64) * self.config.rpm_factor();

        let delta_min = delta_time.get::<minute>();
        let rpm_roda = AV::new::<revolution_per_minute>(roller_revolution / delta_min);
        let rpm_engine = AV::new::<revolution_per_minute>(engine_revolution / delta_min);

        let torque = {
            let g = self.config.perbandingan_gear_beban();
            let inertia = self.config.inertia_roller_beban();
            let percepatan = {
                let rps_roda_now = rpm_roda.get::<radian_per_second>();
                let rps_roda_last = last_data.rpm_roda.get::<radian_per_second>();
                ((rps_roda_now - rps_roda_last) * g) / delta_time.get::<second>()
            };
            Torque::new::<newton_meter>(inertia * percepatan)
        };

        let horsepower = {
            let kecepatan_sudut = rpm_roda.get::<radian_per_second>();
            Power::new::<watt>(torque.get::<newton_meter>() * kecepatan_sudut / 5252.)
        };

        let odo = self.config.circumference_roller::<kilometer>() * roller_revolution;
        let speed = Velocity::new::<kilometer_per_hour>(odo / delta_time.get::<hour>());
        let temp = TemperatureInterval::new::<degree_celsius>(
            raw.temp_celcius().unwrap_or_else(|| self.last().temp.value),
        );

        self.odo_km += odo;
        self.data.push(Data {
            torque,
            horsepower,
            rpm_roda,
            rpm_engine,
            timestamp,
            speed,
            temp,
        });
    }
}

mod si_type {
    pub(super) use uom::si::f64::*;
    pub(super) use AngularVelocity as AV;

    pub(super) use uom::si::angular_velocity::radian_per_second;
    pub(super) use uom::si::angular_velocity::revolution_per_minute;
    pub(super) use uom::si::length::kilometer;
    pub(super) use uom::si::length::meter;
    pub(super) use uom::si::power::watt;
    pub(super) use uom::si::temperature_interval::degree_celsius;
    pub(super) use uom::si::time::hour;
    pub(super) use uom::si::time::minute;
    pub(super) use uom::si::time::second;
    pub(super) use uom::si::torque::newton_meter;
    pub(super) use uom::si::velocity::kilometer_per_hour;
}

#[cfg(test)]
mod tests {}
