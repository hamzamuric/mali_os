#![no_std]
#![no_main]

use core::panic::PanicInfo;
use mali_os::{QemuExitCode, exit_qemu, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("\n===== TESTS =====");
    should_fail();
    serial_println!("[test did not panic]");
    serial_println!("=================");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    serial_println!("=================");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_fail...");
    assert_eq!(0, 1);
}
