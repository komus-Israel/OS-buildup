use core::panic::PanicInfo;

/// Handle panic

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}