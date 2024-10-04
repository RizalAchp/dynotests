use dyno_core::types::RawSerialData;
use embassy_stm32::{bind_interrupts, peripherals, usart};

pub type UsartInstance = peripherals::USART1;
pub type UsartTx = peripherals::PA9;
pub type UsartRx = peripherals::PA10;
pub type UsartCts = peripherals::PA11;
pub type UsartRts = peripherals::PA12;

pub type UsartTxDma = peripherals::DMA2_CH7;
pub type UsartRxDma = peripherals::DMA2_CH5;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

const BUF_SIZE: usize = RawSerialData::BYTE_SIZE * 2;
pub struct Serial {
    usart: usart::Uart<'static, UsartInstance, UsartTxDma, UsartRxDma>,
    buffer: [u8; BUF_SIZE],
}
impl Serial {
    pub fn new(
        peri: UsartInstance,
        rx: UsartRx,
        tx: UsartTx,
        rts: UsartRts,
        cts: UsartCts,
        tx_dma: UsartTxDma,
        rx_dma: UsartRxDma,
    ) -> Result<Self, usart::ConfigError> {
        let usart = usart::Uart::new_with_rtscts(
            peri,
            rx,
            tx,
            Irqs,
            rts,
            cts,
            tx_dma,
            rx_dma,
            Self::config(),
        )?;
        Ok(Self {
            usart,
            buffer: [0u8; BUF_SIZE],
        })
    }

    #[inline]
    pub async fn read_deserialize<T>(&mut self) -> T
    where
        T: for<'de> dyno_core::serde::Deserialize<'de>,
    {
        let Self { usart, buffer } = self;
        let readed = unwrap!(usart.read_until_idle(buffer).await);
        let (t, used) = unwrap!(postcard::take_from_bytes_cobs(&mut buffer[..readed]));
        let readed = used.len();
        buffer.rotate_left(readed);
        t
    }
    #[inline]
    pub async fn write_serialize<T>(&mut self, value: &T)
    where
        T: dyno_core::serde::Serialize,
    {
        let Self { usart, buffer } = self;
        let used_buffer = unwrap!(postcard::to_slice_cobs(value, buffer));
        unwrap!(usart.write(used_buffer).await)
    }

    #[inline]
    fn config() -> usart::Config {
        let mut cfg = usart::Config::default();
        cfg.baudrate = 512_000;
        cfg.parity = usart::Parity::ParityEven;
        cfg
    }
}
