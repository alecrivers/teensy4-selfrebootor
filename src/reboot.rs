pub fn do_reboot() {
    unsafe { core::arch::asm!("bkpt #251") };
}
