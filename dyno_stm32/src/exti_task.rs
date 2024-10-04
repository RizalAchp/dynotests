use core::sync::atomic::AtomicU16;

use embassy_stm32::{exti, gpio, peripherals};

pub type EncoderInput = peripherals::PA2;
pub type RpmInput = peripherals::PB9;
pub type EncoderExtiChan = <EncoderInput as gpio::Pin>::ExtiChannel;
pub type RpmExtiChan = <RpmInput as gpio::Pin>::ExtiChannel;

pub static ENCODER_PULSE: AtomicU16 = AtomicU16::new(0);
pub static RPM_PULSE: AtomicU16 = AtomicU16::new(0);

#[embassy_executor::task]
pub async fn exti_input_encoder(pin: EncoderInput, ch: EncoderExtiChan) {
    let mut ex = exti::ExtiInput::new(gpio::Input::new(pin, gpio::Pull::Down), ch);
    loop {
        ex.wait_for_rising_edge().await;
        ENCODER_PULSE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    }
}

#[embassy_executor::task]
pub async fn exti_input_rpm(pin: RpmInput, ch: RpmExtiChan) {
    let mut ex = exti::ExtiInput::new(gpio::Input::new(pin, gpio::Pull::Down), ch);
    loop {
        ex.wait_for_rising_edge().await;
        ENCODER_PULSE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    }
}
