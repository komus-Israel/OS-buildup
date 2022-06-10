#![no_std]
#![no_main]

mod panic_handler_file;


/**
 * No dependance on OS and standard library     `#![no_std]`
 * Disable access to the compiler entry point   `#![no_main]`
 */


pub extern "C" fn _start() -> !{
    
    loop {}
    //panic_handler_file::panic();

}
