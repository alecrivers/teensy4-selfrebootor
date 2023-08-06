//! Demonstrates a USB mouse using RTIC.
//!
//! Flash your board with this example. You should observe your mouse slowly
//! inching in one direction every time the LED blinks.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::board;

#[rtic::app(device = teensy4_bsp)]
mod app {
    use super::board;

    use hal::usbd::{BusAdapter, EndpointMemory, EndpointState};
    use imxrt_hal as hal;

    use embedded_hal::serial::Write;
    use usb_device::{
        bus::UsbBusAllocator,
        device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    };
    use usbd_hid::{descriptor::SerializedDescriptor as _, hid_class::HIDClass};

    const LOG_POLL_INTERVAL: u32 = board::PERCLK_FREQUENCY / 1_000;
    const LOG_DMA_CHANNEL: usize = 0;

    /// The USB GPT timer we use to (infrequently) send mouse updates.
    const GPT_INSTANCE: imxrt_usbd::gpt::Instance = imxrt_usbd::gpt::Instance::Gpt0;
    /// How frequently should we push mouse updates to the host?
    const MOUSE_UPDATE_INTERVAL_MS: u32 = 200;

    /// This allocation is shared across all USB endpoints. It needs to be large
    /// enough to hold the maximum packet size for *all* endpoints. If you start
    /// noticing panics, check to make sure that this is large enough for all endpoints.
    static EP_MEMORY: EndpointMemory<1024> = EndpointMemory::new();
    /// This manages the endpoints. It's large enough to hold the maximum number
    /// of endpoints; we're not using all the endpoints in this example.
    static EP_STATE: EndpointState = EndpointState::max_endpoints();

    type Bus = BusAdapter;

    #[local]
    struct Local {
        class: HIDClass<'static, Bus>,
        device: UsbDevice<'static, Bus>,
        led: board::Led,
        poller: imxrt_log::Poller,
        timer: hal::pit::Pit<3>,
    }

    #[shared]
    struct Shared {}

    #[init(local = [bus: Option<UsbBusAllocator<Bus>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Resources {
            pins,
            pit: (_, _, _, mut poll_log),
            mut dma,
            lpuart6,
            usb,
            mut gpio2,
            ..
        } = board::tmm(cx.device);

        // Logging
        let log_dma = dma[LOG_DMA_CHANNEL].take().unwrap();
        let mut log_uart = board::lpuart(lpuart6, pins.p1, pins.p0, 115200);
        nb::block!(log_uart.write(b'\r')).unwrap();
        nb::block!(log_uart.write(b'\n')).unwrap();
        nb::block!(log_uart.flush()).unwrap();
        let log_poller =
            imxrt_log::log::lpuart(log_uart, log_dma, imxrt_log::Interrupts::Enabled).unwrap();
        poll_log.set_interrupt_enable(true);
        poll_log.set_load_timer_value(LOG_POLL_INTERVAL);
        poll_log.enable();

        // Initialize LED
        let led = board::led(&mut gpio2, pins.p13);
        led.set();

        // USB
        let bus = BusAdapter::new(usb, &EP_MEMORY, &EP_STATE);
        bus.set_interrupts(true);
        bus.gpt_mut(GPT_INSTANCE, |gpt| {
            gpt.stop();
            gpt.clear_elapsed();
            gpt.set_interrupt_enabled(true);
            gpt.set_mode(imxrt_usbd::gpt::Mode::Repeat);
            gpt.set_load(MOUSE_UPDATE_INTERVAL_MS * 1000);
            gpt.reset();
            gpt.run();
        });

        let bus = cx.local.bus.insert(UsbBusAllocator::new(bus));
        // Note that "4" correlates to a 1ms polling interval. Since this is a high speed
        // device, bInterval is computed differently.
        let class = HIDClass::new(
            bus,
            teensy4_selfrebootor::hid_descriptor::Rebootor::desc(),
            10,
        );
        let device = UsbDeviceBuilder::new(bus, UsbVidPid(0x16C0, 0x0477))
            .product("Rebootor")
            .manufacturer("PJRC")
            .self_powered(true)
            .build();

        (
            Shared {},
            Local {
                class,
                device,
                led,
                poller: log_poller,
                timer: poll_log,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = PIT, local = [poller, timer], priority = 1)]
    fn pit_interrupt(ctx: pit_interrupt::Context) {
        while ctx.local.timer.is_elapsed() {
            ctx.local.timer.clear_elapsed();
        }

        ctx.local.poller.poll();
    }

    #[task(binds = USB_OTG1, local = [device, class, led, configured: bool = false], priority = 2)]
    fn usb1(ctx: usb1::Context) {
        let usb1::LocalResources {
            class,
            device,
            led,
            configured,
        } = ctx.local;

        device.poll(&mut [class]);

        if device.state() == UsbDeviceState::Configured {
            if !*configured {
                device.bus().configure();
            }
            *configured = true;
        } else {
            *configured = false;
        }

        if *configured {
            let elapsed = device.bus().gpt_mut(GPT_INSTANCE, |gpt| {
                let elapsed = gpt.is_elapsed();
                while gpt.is_elapsed() {
                    gpt.clear_elapsed();
                }
                elapsed
            });

            let mut buf = [0u8; 20];
            let result = class.pull_raw_output(&mut buf);
            match result {
                Ok(info) => {
                    log::info!("Data received: {:?}", info);
                    let buf = &buf[..info];
                    log::info!("Data: {:?}", core::str::from_utf8(buf));
                    if buf == b"reboot" {
                        unsafe { core::arch::asm!("bkpt #251") };
                    }
                }
                Err(usb_device::UsbError::WouldBlock) => (),
                Err(e) => {
                    log::info!("Report error: {:?}", e);
                }
            }
            if elapsed {
                led.toggle();
            }
        }
    }
}
