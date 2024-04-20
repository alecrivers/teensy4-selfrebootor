// This example demonstrates a program that also acts as a rebootor.
// When being programmed with the `-r` flag, it will reboot itself into
// the bootloader.
// In addition, it runs as a USB serial port, echoing back any data sent to it.
#![no_std]
#![no_main]

use teensy4_bsp as bsp;

#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {
    ::teensy4_panic::sos()
}

use rtic_monotonics::imxrt::prelude::*;
use usbd_serial::CustomControlRequestHandler;
imxrt_gpt1_monotonic!(Mono, teensy4_bsp::board::PERCLK_FREQUENCY);

#[rtic::app(device = teensy4_bsp, dispatchers = [CAN1, CAN2])]
mod app {
    use super::*;

    use bsp::board;
    use bsp::hal;

    use hal::usbd::{BusAdapter, EndpointMemory, EndpointState, Speed};
    use usb_device::bus::UsbBusAllocator;

    use usb_device::device::UsbDevice;
    use usb_device::device::UsbDeviceBuilder;
    use usb_device::device::UsbDeviceState;
    use usb_device::device::UsbVidPid;
    use usbd_serial::SerialPort;

    /// This allocation is shared across all USB endpoints. It needs to be large
    /// enough to hold the maximum packet size for *all* endpoints. If you start
    /// noticing panics, check to make sure that this is large enough for all endpoints.
    static EP_MEMORY: EndpointMemory<2048> = EndpointMemory::new();
    /// This manages the endpoints. It's large enough to hold the maximum number
    /// of endpoints; we're not using all the endpoints in this example.
    static EP_STATE: EndpointState = EndpointState::max_endpoints();

    struct RebootRequestHandler {
    }
    impl CustomControlRequestHandler for RebootRequestHandler {
        fn handle_request(&mut self, _req: &usb_device::control::Request) -> bool {
            // Don't reboot right away, because we need to acknowledge the request first (I think).
            reboot_soon::spawn().ok();
            true
        }
    }

    #[local]
    struct Local {
        led: board::Led,
        class: SerialPort<'static, BusAdapter, RebootRequestHandler>,
        device: UsbDevice<'static, BusAdapter>,
    }

    #[shared]
    struct Shared {
    }

    #[init(local = [bus: Option<UsbBusAllocator<BusAdapter>> = None])]
    // #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb,
            pins,
            mut gpio2,
            mut gpt1,
            ..
        } = board::t41(cx.device);

        // Setup clock
        gpt1.set_clock_source(hal::gpt::ClockSource::PeripheralClock);
        Mono::start(gpt1.release());

        // Setup USB
        let bus = BusAdapter::with_speed(usb, &EP_MEMORY, &EP_STATE, Speed::High);
        let bus = cx.local.bus.insert(UsbBusAllocator::new(bus));
        let class = SerialPort::new(bus, RebootRequestHandler {});
        // Note the UsbVidPid must be those in order to be recognized by the Teensy loader as a rebootor
        let device = UsbDeviceBuilder::new(bus, UsbVidPid(0x16C0, 0x0477))
            .product("Teensy rebootor with USB serial")
            .manufacturer("PJRC")
            .self_powered(true)
            .max_packet_size_0(64)
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();
        device.bus().set_interrupts(true);

        // Setup LED
        let led = board::led(&mut gpio2, pins.p13);
        led.set();

        (
            Shared {
            },
            Local {
                led,
                class,
                device,
            },
        )
    }

    #[task(binds = USB_OTG1, priority = 5, local = [class, device, led, configured: bool = false])]
    fn usb1(ctx: usb1::Context) {
        let usb1::LocalResources {
            class,
            device,
            configured,
            led,
            ..
        } = ctx.local;

        if device.poll(&mut [class]) {
            if device.state() == UsbDeviceState::Configured {
                if !*configured {
                    device.bus().configure();
                }
                *configured = true;

                let mut buffer = [0; 64];
                
                match class.read(&mut buffer) {
                    Ok(count) => {
                        // Toggle LED and also do 2x loopback.
                        led.toggle();
                        class.write(&buffer[..count]).ok();
                        class.write(&buffer[..count]).ok();
                    }
                    Err(usb_device::UsbError::WouldBlock) => {}
                    Err(err) => log::error!("{:?}", err),
                }
            } else {
                *configured = false;
            }
        }
    }

    #[task(priority = 1)]
    async fn reboot_soon(_cx: reboot_soon::Context) {
        Mono::delay(10.millis()).await;
        teensy4_selfrebootor::reboot_to_bootloader();
    }
}
