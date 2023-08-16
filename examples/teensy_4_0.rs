// This example demonstrates a program that also acts as a rebootor.
// When being programmed with the `-r` flag, it will reboot itself into
// the bootloader.

#![no_std]
#![no_main]

mod common;
common::uart_panic_handler!(lpuart6, p1, p0, 115200);

#[rtic::app(device = teensy4_bsp)]
mod app {
    use teensy4_bsp as bsp;

    use bsp::board;
    use bsp::hal;
    use bsp::logging;

    use embedded_hal::serial::Write;

    use hal::usbd::{BusAdapter, EndpointMemory, EndpointState, Speed};
    use usb_device::bus::UsbBusAllocator;

    use teensy4_selfrebootor::Rebootor;

    /// This allocation is shared across all USB endpoints. It needs to be large
    /// enough to hold the maximum packet size for *all* endpoints. If you start
    /// noticing panics, check to make sure that this is large enough for all endpoints.
    static EP_MEMORY: EndpointMemory<1024> = EndpointMemory::new();
    /// This manages the endpoints. It's large enough to hold the maximum number
    /// of endpoints; we're not using all the endpoints in this example.
    static EP_STATE: EndpointState = EndpointState::max_endpoints();

    const LOG_POLL_INTERVAL: u32 = board::PERCLK_FREQUENCY / 100;
    const LOG_DMA_CHANNEL: usize = 0;

    #[local]
    struct Local {
        poll_log: hal::pit::Pit<3>,
        log_poller: logging::Poller,
        rebootor: Rebootor<'static>,
        led: board::Led,
    }

    #[shared]
    struct Shared {}

    #[init(local = [bus: Option<UsbBusAllocator<BusAdapter>> = None])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut dma,
            pit: (_, _, _, mut poll_log),
            pins,
            usb,
            mut gpio2,
            lpuart6,
            ..
        } = board::t40(cx.device);

        // Logging
        let log_dma = dma[LOG_DMA_CHANNEL].take().unwrap();
        let mut log_uart = board::lpuart(lpuart6, pins.p1, pins.p0, 115200);
        for &ch in "\r\n===== Rebootor example =====\r\n\r\n".as_bytes() {
            nb::block!(log_uart.write(ch)).unwrap();
        }
        nb::block!(log_uart.flush()).unwrap();
        let log_poller =
            logging::log::lpuart(log_uart, log_dma, logging::Interrupts::Enabled).unwrap();
        poll_log.set_interrupt_enable(true);
        poll_log.set_load_timer_value(LOG_POLL_INTERVAL);
        poll_log.enable();

        // Initialize LED
        let led = board::led(&mut gpio2, pins.p13);
        led.set();

        // USB
        let bus = BusAdapter::with_speed(usb, &EP_MEMORY, &EP_STATE, Speed::LowFull);
        bus.set_interrupts(true);
        let bus = cx.local.bus.insert(UsbBusAllocator::new(bus));
        let rebootor = teensy4_selfrebootor::Rebootor::new(bus);

        (
            Shared {},
            Local {
                log_poller,
                poll_log,
                rebootor,
                led,
            },
        )
    }

    #[task(binds = USB_OTG1, local = [rebootor, led], priority = 5)]
    fn usb1(ctx: usb1::Context) {
        let usb1::LocalResources { rebootor, led, .. } = ctx.local;

        rebootor.poll();
        led.toggle();
    }

    #[task(binds = PIT, priority = 1, local = [poll_log, log_poller])]
    fn blink_and_log(cx: blink_and_log::Context) {
        let blink_and_log::LocalResources {
            poll_log,
            log_poller,
            ..
        } = cx.local;

        if poll_log.is_elapsed() {
            poll_log.clear_elapsed();
            log_poller.poll();
        }
    }
}
