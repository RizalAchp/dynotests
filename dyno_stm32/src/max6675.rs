use embassy_stm32::{
    gpio, peripherals,
    spi::{self, BitOrder, MODE_0},
    time::mhz,
};
pub type Max6675Instance = peripherals::SPI1;
pub type Max6675SCK = peripherals::PA5;
pub type Max6675MISO = peripherals::PA6;
pub type Max6675CS = peripherals::PB0;
pub type Max6675TxDma = peripherals::DMA2_CH2;
pub type Max6675RxDma = peripherals::DMA2_CH0;

pub struct Max6675 {
    spi: spi::Spi<'static, Max6675Instance, Max6675TxDma, Max6675RxDma>,
    cs: gpio::Output<'static, Max6675CS>,
}

impl Max6675 {
    pub fn new(
        peri: Max6675Instance,
        sck: Max6675SCK,
        miso: Max6675MISO,
        cs: Max6675CS,
        txdma: Max6675TxDma,
        rxdma: Max6675RxDma,
    ) -> Self {
        let cs = gpio::Output::new(cs, gpio::Level::High, gpio::Speed::Low);
        let spi = spi::Spi::new_rxonly(peri, sck, miso, txdma, rxdma, Self::config());

        Self { spi, cs }
    }
    #[inline]
    pub async fn read_temp_raw(&mut self) -> u16 {
        let Self { spi, cs } = self;
        let mut buf = [0u8; 2];
        cs.set_low();
        let _ = spi.read(&mut buf).await;
        cs.set_high();
        u16::from_be_bytes(buf)
    }

    #[inline]
    fn config() -> spi::Config {
        let mut cfg = spi::Config::default();

        cfg.mode = MODE_0;
        cfg.bit_order = BitOrder::MsbFirst;
        cfg.frequency = mhz(4);
        cfg
    }
}
