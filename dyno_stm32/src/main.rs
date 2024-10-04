#![no_main]
#![no_std]

use core::sync::atomic::Ordering;

#[cfg(feature = "defmt")]
use defmt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
use panic_probe as _;

use dyno_core::types::{RawSerialData, SerialDataInit};
use exti_task::{ENCODER_PULSE, RPM_PULSE};
#[cfg(not(feature = "defmt"))]
#[panic_handler]
fn panic_abort(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}

#[macro_use]
mod macros;

mod exti_task;
mod max6675;
mod serial;

// pub type LedIndicatorA = gpio::Output<'static, peripherals::PB3>;
// pub type LedIndicatorB = gpio::Output<'static, peripherals::PB4>;

#[embassy_executor::task]
async fn main_task(mut serial: serial::Serial, mut max6675: max6675::Max6675) {
    let mut raw_data = RawSerialData::new();
    let init = serial.read_deserialize::<SerialDataInit>().await;
    let ticker_duration = embassy_time::Duration::from_millis(init.period_ms() as _);
    let mut ticker = embassy_time::Ticker::every(ticker_duration);
    loop {
        ticker.next().await;
        raw_data.raw_temp = max6675.read_temp_raw().await;
        raw_data.pulse_rpm = RPM_PULSE.swap(0, Ordering::Relaxed);
        raw_data.pulse_enc = ENCODER_PULSE.swap(0, Ordering::Relaxed);
        serial.write_serialize(&raw_data).await;
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut executor = ::embassy_executor::Executor::new();
    let ph = embassy_stm32::init(config());

    __make_static(&mut executor).run(move |spawner| {
        // let led_a: LedIndicatorA = gpio::Output::new(ph.PB3, Level::High, Speed::Low);
        // let led_b: LedIndicatorB = gpio::Output::new(ph.PB4, Level::High, Speed::Low);
        let serial = serial::Serial::new(
            ph.USART1,   // serial instance
            ph.PA10,     // rx
            ph.PA9,      // tx
            ph.PA12,     // rts
            ph.PA11,     // cts
            ph.DMA2_CH7, // txdma
            ph.DMA2_CH5, // rxdma
        );
        let max6675 = max6675::Max6675::new(
            ph.SPI1,     // spi instance
            ph.PA5,      // sck
            ph.PA6,      // miso
            ph.PB0,      // cs
            ph.DMA2_CH2, // txdma (unused b'cs max6675 is rx only spi )
            ph.DMA2_CH0, // rxdma
        );

        unwrap!(spawner.spawn(exti_task::exti_input_rpm(ph.PB9, ph.EXTI9)));
        unwrap!(spawner.spawn(exti_task::exti_input_encoder(ph.PA2, ph.EXTI2)));
        unwrap!(spawner.spawn(main_task(serial, max6675)))
    })
}

fn config() -> embassy_stm32::Config {
    embassy_stm32::Config::default()
}

fn __make_static<T>(t: &mut T) -> &'static mut T {
    unsafe { core::mem::transmute(t) }
}
