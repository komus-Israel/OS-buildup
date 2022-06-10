use core::panic::PanicInfo;

/// panic handler

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}