#![no_std]

mod panic_handler_file;


/**
 * No dependance on OS and standard library
 */


fn main() {
    
    panic_handler_file::panic;

}
