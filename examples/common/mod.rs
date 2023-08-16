macro_rules! uart_panic_handler {
    ($uart: ident, $tx_pin: ident, $rx_pin: ident, $baud: expr) => {
        #[panic_handler]
        fn panic(info: &::core::panic::PanicInfo) -> ! {
            use ::core::fmt::Write as _;
            use ::embedded_hal::serial::Write as _;

            let ::teensy4_bsp::board::Resources {
                $uart: uart, pins, ..
            } = ::teensy4_bsp::board::tmm(unsafe { ::teensy4_bsp::ral::Instances::instances() });

            let uart = ::teensy4_bsp::board::lpuart(uart, pins.$tx_pin, pins.$rx_pin, $baud);

            struct UartWriter<P, const N: u8> {
                uart: ::teensy4_bsp::hal::lpuart::Lpuart<P, N>,
            }
            impl<P, const N: u8> ::core::fmt::Write for UartWriter<P, N> {
                fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
                    for b in s.as_bytes() {
                        let _ = ::nb::block!(self.uart.write(*b));
                    }
                    Ok(())
                }
            }

            let mut uart = UartWriter { uart };

            ::core::write!(uart, "\r\n{}\r\n", info).ok();
            let _ = ::nb::block!(uart.uart.flush());

            ::teensy4_panic::sos()
        }
    };
}

pub(crate) use uart_panic_handler;
