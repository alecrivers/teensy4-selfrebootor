#![no_std]
//#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/Finomnis/teensy4-selfrebootor/issues")]
#![cfg_attr(docsrs, feature(doc_cfg))]

use imxrt_usbd::BusAdapter;
use usb_device::{class_prelude::*, prelude::*};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

mod hid_descriptor;
mod reboot;

pub struct Rebootor {
    class: HIDClass<'static, BusAdapter>,
    device: UsbDevice<'static, BusAdapter>,
    configured: bool,
}

impl Rebootor {
    pub fn new(bus_alloc: &'static UsbBusAllocator<BusAdapter>) -> Self {
        let class = HIDClass::new(bus_alloc, crate::hid_descriptor::Rebootor::desc(), 10);
        let device = UsbDeviceBuilder::new(bus_alloc, UsbVidPid(0x16C0, 0x0477))
            .product("Self-Rebootor")
            .manufacturer("PJRC")
            .self_powered(true)
            .build();

        Self {
            class,
            device,
            configured: false,
        }
    }

    pub fn poll(&mut self) {
        self.device.poll(&mut [&mut self.class]);

        if self.device.state() == UsbDeviceState::Configured {
            if !self.configured {
                self.device.bus().configure();
            }
            self.configured = true;
        } else {
            self.configured = false;
        }

        if self.configured {
            let mut buf = [0u8; 6];

            let result = self.class.pull_raw_output(&mut buf);
            match result {
                Ok(info) => {
                    let buf = &buf[..info];
                    if buf == b"reboot" {
                        reboot::do_reboot();
                    }
                }
                Err(usb_device::UsbError::WouldBlock) => (),
                Err(_) => {}
            }
        }
    }
}
