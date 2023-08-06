// This example demonstrates a program that also acts as a rebootor.
// When being programmed with the `-r` flag, it will reboot itself into
// the bootloader.

#![no_std]
#![no_main]

mod common;

use teensy4_bsp as bsp;

#[rtic::app(device = teensy4_bsp)]
mod app {
    use super::bsp;
    use bsp::board;
    use bsp::hal;

    use hal::usbd::{BusAdapter, EndpointMemory, EndpointState};
    use usb_device::bus::UsbBusAllocator;

    use teensy4_selfrebootor::Rebootor;

    use crate::common::uart::{uart_log, UartWriter};

    /// This allocation is shared across all USB endpoints. It needs to be large
    /// enough to hold the maximum packet size for *all* endpoints. If you start
    /// noticing panics, check to make sure that this is large enough for all endpoints.
    static EP_MEMORY: EndpointMemory<1024> = EndpointMemory::new();
    /// This manages the endpoints. It's large enough to hold the maximum number
    /// of endpoints; we're not using all the endpoints in this example.
    static EP_STATE: EndpointState = EndpointState::max_endpoints();

    #[local]
    struct Local {
        rebootor: Rebootor<'static>,
        led: board::Led,
    }

    #[shared]
    struct Shared {}

    #[init(local = [bus: Option<UsbBusAllocator<BusAdapter>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Resources {
            pins,
            usb,
            mut gpio2,
            lpuart6,
            ..
        } = board::tmm(cx.device);

        // Initialize UART
        let mut uart = UartWriter::new(board::lpuart(lpuart6, pins.p1, pins.p0, 115200));
        writeln!(uart);

        // Write welcome message
        writeln!(uart, "===== Rebootor example =====");
        writeln!(uart);

        // Initialize logging
        uart_log::init(uart, log::LevelFilter::Debug);

        // Initialize LED
        let led = board::led(&mut gpio2, pins.p13);
        led.set();

        // USB
        let bus = BusAdapter::new(usb, &EP_MEMORY, &EP_STATE);
        bus.set_interrupts(true);
        let bus = cx.local.bus.insert(UsbBusAllocator::new(bus));
        let rebootor = teensy4_selfrebootor::Rebootor::new(bus);

        (Shared {}, Local { rebootor, led }, init::Monotonics())
    }

    #[task(binds = USB_OTG1, local = [rebootor, led], priority = 2)]
    fn usb1(ctx: usb1::Context) {
        let usb1::LocalResources { rebootor, led } = ctx.local;

        rebootor.poll();
        led.toggle();
    }
}
