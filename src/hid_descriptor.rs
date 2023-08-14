use usbd_hid::descriptor::generator_prelude::*;

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
