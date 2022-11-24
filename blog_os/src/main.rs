#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

// By removing the standard library there is a function called start that points the compiler to the starting point of the application that is missing. 
// We need to redifine that here.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop{}
} 

// since the panic handler requires the standard library, we need to redeclare it here
#[panic_handler]
fn panic( _info: &PanicInfo ) -> ! {
    loop {}
}