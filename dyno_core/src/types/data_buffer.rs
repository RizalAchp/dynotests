use core::cmp::Ordering;
use core::ops;
use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use chrono::{DateTime, Duration, Utc};
use derive_more::Display;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

use super::{calculate_odo_from_data, calculate_odo_from_datas, Data};
use crate::config::DynoConfig;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "derive_serde",
    derive(Deserialize, Serialize),
    serde(default)
)]
#[derive(Display)]
#[display("{self:#?}")]
pub struct DynotestData {
    pub config: DynoConfig,

    pub(crate) data: Vec<Data>,

    pub(crate) start_time: DateTime<Utc>,
    pub(crate) odo_km: f64,
}
const BUFFER_DATA_INIT_CAP: usize = 1800;

impl DynotestData {
    pub fn new(config: DynoConfig) -> Self {
        Self {
            start_time: DateTime::UNIX_EPOCH,
            config,
            data: Vec::with_capacity(BUFFER_DATA_INIT_CAP),
            odo_km: 0.,
        }
    }

    #[inline(always)]
    pub fn start(&mut self) {
        self.start_time = Utc::now();
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.data.clear();
        self.data.push(Data::default());
        self.odo_km = 0.;
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn push(&mut self, data: Data) {
        let last_timestamp = self.last().timestamp_date_time();
        let odo = calculate_odo_from_data(&last_timestamp, &data);
        self.odo_km += odo.value;
        self.data.push(data);
    }

    #[inline]
    pub fn add_odo_km(&mut self, odo: f64) {
        self.odo_km += odo;
    }

    #[inline]
    pub fn total_time_format(&self) -> Option<String> {
        let time = self.total_time();
        let ms = time.num_milliseconds();

        let ms = ms % 1000;
        let s = (ms / 1000) % 60;
        let m = (ms / (1000 * 60)) % 60;
        let h = (ms / (1000 * 60 * 60)) % 24;
        Some(format!("{h:02}:{m:02}:{s:02}.{ms:03}"))
    }
    #[inline]
    pub fn total_time(&self) -> Duration {
        let first = self.last().timestamp_date_time();
        let last = self.first().timestamp_date_time();
        last - first
    }

    #[inline]
    pub fn last(&self) -> &Data {
        self.data.last().unwrap_or(&Data::ZERO)
    }

    #[inline]
    pub fn first(&self) -> &Data {
        self.data.first().unwrap_or(&Data::ZERO)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    pub fn max(&self) -> &Data {
        self.max_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal))
    }

    #[inline(always)]
    pub fn min(&self) -> &Data {
        self.min_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal))
    }

    #[inline(always)]
    pub fn max_by(&self, compare: impl FnMut(&&Data, &&Data) -> Ordering) -> &Data {
        self.data.iter().max_by(compare).unwrap_or(&Data::ZERO)
    }
    #[inline(always)]
    pub fn min_by(&self, compare: impl FnMut(&&Data, &&Data) -> Ordering) -> &Data {
        self.data.iter().min_by(compare).unwrap_or(&Data::ZERO)
    }
}

impl Default for DynotestData {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
impl ops::Deref for DynotestData {
    type Target = Vec<Data>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(feature = "std")]
impl Extend<Data> for DynotestData {
    fn extend<T: IntoIterator<Item = Data>>(&mut self, iter: T) {
        use uom::si::length::kilometer;
        let last_time = self.last().timestamp_date_time();

        self.data.extend(iter);
        self.odo_km += calculate_odo_from_datas(last_time, &self.data).get::<kilometer>();
    }
}

#[cfg(feature = "std")]
impl FromIterator<Data> for DynotestData {
    fn from_iter<T: IntoIterator<Item = Data>>(iter: T) -> Self {
        use uom::si::length::kilometer;
        let data = Vec::from_iter(iter);
        let start_time = data
            .first()
            .and_then(|x| DateTime::from_timestamp_millis(x.timestamp))
            .unwrap_or_default();
        let odo_km = calculate_odo_from_datas(start_time, &data).get::<kilometer>();
        Self {
            data,
            odo_km,
            start_time,
            config: Default::default(),
        }
    }
}

impl<I: SliceIndex<[Data]>> Index<I> for DynotestData {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        self.data.index(index)
    }
}
impl<I: SliceIndex<[Data]>> IndexMut<I> for DynotestData {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::sync::OnceLock;

    use uom::si::f64::*;
    use uom::si::{
        angular_velocity::revolution_per_minute, power::horsepower_metric,
        temperature_interval::degree_celsius, torque::newton_meter, velocity::kilometer_per_hour,
    };

    use crate::types::{Data, DynotestData, RawSerialData};

    const SIZE_TESTED: usize = 5;

    const SER_DATA: RawSerialData = RawSerialData {
        pulse_enc: 10,
        pulse_rpm: 20,
        raw_temp: 420,
    };

    fn data_buffer() -> &'static DynotestData {
        static BUFFER_DATA: OnceLock<DynotestData> = OnceLock::new();
        BUFFER_DATA.get_or_init(move || {
            let mut buffer = DynotestData::default();
            for _ in 0..SIZE_TESTED {
                buffer.push_from_raw_serial_data(SER_DATA);
            }
            buffer
        })
    }

    fn assert_data(data: Data, expect: Data) {
        let Data {
            speed,
            torque,
            horsepower,
            rpm_roda,
            rpm_engine,
            ..
        } = data;
        assert_eq!(
            speed.get::<kilometer_per_hour>().floor(),
            expect.speed.get::<kilometer_per_hour>().round(),
            "data speed asserts"
        );
        assert_eq!(
            torque.get::<newton_meter>().floor(),
            expect.torque.get::<newton_meter>().round(),
            "data torque asserts"
        );
        assert_eq!(
            horsepower.get::<horsepower_metric>().floor(),
            expect.horsepower.get::<horsepower_metric>().round(),
            "data horsepower asserts"
        );
        assert_eq!(
            rpm_roda.get::<revolution_per_minute>().floor(),
            expect.rpm_roda.get::<revolution_per_minute>().round(),
            "data rpm roda asserts"
        );
        assert_eq!(
            rpm_engine.get::<revolution_per_minute>().floor(),
            expect.rpm_engine.get::<revolution_per_minute>().round(),
            "data rpm engine asserts"
        );
    }

    fn data(speed: f64, torque: f64, horsepower: f64, rpm_roda: f64, rpm_engine: f64) -> Data {
        Data {
            speed: Velocity::new::<kilometer_per_hour>(speed),
            torque: Torque::new::<newton_meter>(torque),
            horsepower: Power::new::<horsepower_metric>(horsepower),
            rpm_roda: AngularVelocity::new::<revolution_per_minute>(rpm_roda),
            rpm_engine: AngularVelocity::new::<revolution_per_minute>(rpm_engine),
            temp: TemperatureInterval::new::<degree_celsius>(420.),
            timestamp: Default::default(),
        }
    }

    #[test]
    fn test_data_buffer() {
        let buffer = data_buffer();
        assert_eq!(buffer.len(), SIZE_TESTED);
        assert_data(buffer[0], data(64.0, 93., 0., 2400., 9600.));
    }
}
