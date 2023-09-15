// This example demonstrates a program that also acts as a rebootor.
// When being programmed with the `-r` flag, it will reboot itself into
// the bootloader.

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {
    ::teensy4_panic::sos()
}

#[rtic::app(device = teensy4_bsp)]
mod app {
    use teensy4_bsp::hal::usbd::{BusAdapter, EndpointMemory, EndpointState};
    use usb_device::bus::UsbBusAllocator;

    static EP_MEMORY: EndpointMemory<1024> = EndpointMemory::new();
    static EP_STATE: EndpointState<4> = EndpointState::new();

    #[local]
    struct Local {
        rebootor: teensy4_selfrebootor::Rebootor<'static>,
    }

    #[shared]
    struct Shared {}

    #[init(local = [bus: Option<UsbBusAllocator<BusAdapter>> = None])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let teensy4_bsp::board::Resources { usb, .. } = teensy4_bsp::board::t40(cx.device);

        let usb_bus = cx.local.bus.insert(UsbBusAllocator::new(BusAdapter::new(
            usb, &EP_MEMORY, &EP_STATE,
        )));

        let rebootor = teensy4_selfrebootor::Rebootor::new(usb_bus);

        (Shared {}, Local { rebootor })
    }

    #[task(binds = USB_OTG1, priority = 5, local = [rebootor])]
    fn usb1(ctx: usb1::Context) {
        ctx.local.rebootor.poll();
    }
}
