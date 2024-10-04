#![no_main]
#![no_std]

mod max6675;
mod serial;
mod task;

#[macro_use]
extern crate defmt;

use defmt_rtt as _;
use panic_probe as _;

use embassy_stm32::{gpio, peripherals};

use dyno_core::types::{RawSerialData, SerialDataInit};
use task::GLOBAL_DATA;

type IndicatorA = gpio::Output<'static, peripherals::PB3>;
type IndicatorB = gpio::Output<'static, peripherals::PB4>;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut executor = ::embassy_executor::Executor::new();
    let ph = embassy_stm32::init(config());
    let mut indicator_a = gpio::Output::new(ph.PB3, gpio::Level::Low, gpio::Speed::Low);
    let mut indicator_b = gpio::Output::new(ph.PB4, gpio::Level::Low, gpio::Speed::Low);
    for _ in 0..3 {
        indicator_a.set_high();
        embassy_time::block_for(embassy_time::Duration::from_millis(500));
        indicator_a.set_low();
        embassy_time::block_for(embassy_time::Duration::from_millis(500));
        indicator_b.set_high();
        embassy_time::block_for(embassy_time::Duration::from_millis(500));
        indicator_b.set_low();
        embassy_time::block_for(embassy_time::Duration::from_millis(500));
    }

    indicator_b.set_high();

    __make_static(&mut executor).run(move |spawner| {
        // let led_a: LedIndicatorA = gpio::Output::new(ph.PB3, Level::High, Speed::Low);
        // let led_b: LedIndicatorB = gpio::Output::new(ph.PB4, Level::High, Speed::Low);
        let serial = match serial::Serial::new(
            ph.USART1,   // serial instance
            ph.PA10,     // rx
            ph.PA9,      // tx
            ph.PA12,     // rts
            ph.PA11,     // cts
            ph.DMA2_CH7, // txdma
            ph.DMA2_CH5, // rxdma
        ) {
            Ok(ok) => ok,
            Err(err) => error_indicator(err, indicator_b),
        };
        let max6675 = max6675::Max6675::new(
            ph.SPI1,     // spi instance
            ph.PA5,      // sck
            ph.PA6,      // miso
            ph.PB0,      // cs
            ph.DMA2_CH2, // txdma (unused b'cs max6675 is rx only SPI)
            ph.DMA2_CH0, // rxdma
        );

        if let Err(err) = spawner.spawn(task::exti_input_rpm(ph.PB9, ph.EXTI9)) {
            error_indicator(err, indicator_b);
        }
        if let Err(err) = spawner.spawn(task::exti_input_encoder(ph.PA2, ph.EXTI2)) {
            error_indicator(err, indicator_b);
        }
        if let Err(err) = spawner.spawn(task::max6675_task(max6675)) {
            error_indicator(err, indicator_b);
        }
        if let Err(err) = spawner.spawn(main_task(serial, indicator_a)) {
            error_indicator(err, indicator_b);
        }
    })
}

#[embassy_executor::task]
async fn main_task(mut serial: serial::Serial, mut leda: IndicatorA) {
    let mut raw_data = RawSerialData::new();
    let init = serial.read_deserialize::<SerialDataInit>().await;
    let ticker_duration = embassy_time::Duration::from_millis(init.period_ms() as _);
    let mut ticker = embassy_time::Ticker::every(ticker_duration);
    loop {
        leda.toggle();
        ticker.next().await;
        raw_data.raw_temp = GLOBAL_DATA.get_temp();
        raw_data.pulse_rpm = GLOBAL_DATA.take_rpm();
        raw_data.pulse_enc = GLOBAL_DATA.take_encoder();
        serial.write_serialize(&raw_data).await;
    }
}

#[inline(always)]
fn error_indicator<F: defmt::Format>(err: F, mut b: IndicatorB) -> ! {
    for _ in 0..10 {
        b.toggle();
        embassy_time::block_for(embassy_time::Duration::from_millis(500));
    }
    panic!("ERROR: {}", err)
}

fn config() -> embassy_stm32::Config {
    embassy_stm32::Config::default()
}

fn __make_static<T>(t: &mut T) -> &'static mut T {
    unsafe { core::mem::transmute(t) }
}
