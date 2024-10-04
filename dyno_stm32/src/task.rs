use core::sync::atomic::{AtomicU16, Ordering::Relaxed};

use embassy_stm32::{exti, gpio, peripherals};

pub type EncoderInput = peripherals::PA2;
pub type RpmInput = peripherals::PB9;
pub type EncoderExtiChan = <EncoderInput as gpio::Pin>::ExtiChannel;
pub type RpmExtiChan = <RpmInput as gpio::Pin>::ExtiChannel;

pub struct GlobalData {
    encoder_pulse: AtomicU16,
    rpm_pulse: AtomicU16,

    temp: AtomicU16,
}

pub static GLOBAL_DATA: GlobalData = GlobalData::new();

#[embassy_executor::task]
pub async fn exti_input_encoder(pin: EncoderInput, ch: EncoderExtiChan) {
    let mut ex = exti::ExtiInput::new(gpio::Input::new(pin, gpio::Pull::Down), ch);
    loop {
        ex.wait_for_rising_edge().await;
        GLOBAL_DATA.incr_rpm();
    }
}

#[embassy_executor::task]
pub async fn exti_input_rpm(pin: RpmInput, ch: RpmExtiChan) {
    let mut ex = exti::ExtiInput::new(gpio::Input::new(pin, gpio::Pull::Down), ch);
    loop {
        ex.wait_for_rising_edge().await;
        GLOBAL_DATA.incr_encoder();
    }
}

#[embassy_executor::task]
pub async fn max6675_task(mut ins: crate::max6675::Max6675) {
    loop {
        embassy_time::Timer::after_millis(500).await;
        GLOBAL_DATA.set_temp(ins.read_temp_raw().await);
    }
}

impl GlobalData {
    const fn new() -> Self {
        Self {
            encoder_pulse: AtomicU16::new(0),
            rpm_pulse: AtomicU16::new(0),
            temp: AtomicU16::new(0),
        }
    }

    #[inline]
    fn incr_encoder(&self) -> u16 {
        self.encoder_pulse.fetch_add(1, Relaxed)
    }
    #[inline]
    fn incr_rpm(&self) -> u16 {
        self.rpm_pulse.fetch_add(1, Relaxed)
    }
    #[inline]
    fn set_temp(&self, temp: u16) {
        self.temp.store(temp, Relaxed)
    }
    #[inline]
    pub fn get_temp(&self) -> u16 {
        self.temp.load(Relaxed)
    }

    #[inline]
    pub fn take_encoder(&self) -> u16 {
        self.encoder_pulse.swap(0, Relaxed)
    }
    #[inline]
    pub fn take_rpm(&self) -> u16 {
        self.encoder_pulse.swap(0, Relaxed)
    }
}
