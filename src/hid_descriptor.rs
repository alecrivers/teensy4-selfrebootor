use usbd_hid::descriptor::generator_prelude::*;

/// MouseReport describes a report and its companion descriptor than can be used
/// to send mouse movements and button presses to a host.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = VENDOR_DEFINED_START, usage = 0x0100) = {
        (usage = 0x02,) = {
            output_buffer=output;
        };
    }
)]
pub struct Rebootor {
    output_buffer: [u8; 6],
}
