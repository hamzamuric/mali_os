#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mali_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mali_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Ja sam mali kernel");
    println!("By: Hamza Muric");

    mali_os::init();

    #[cfg(test)]
    test_main();

    println!("Radi!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mali_os::test_panic_handler(info);
}
