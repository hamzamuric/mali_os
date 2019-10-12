#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use mali_os::{QemuExitCode, exit_qemu, serial_print, serial_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(mali_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64
) {
    serial_println!("[ok]");
    serial_println!("=================");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("\n===== TESTS =====");
    serial_print!("stack_overflow... ");

    mali_os::gdt::init();
    init_test_idt();

    stack_overflow();

    serial_println!("[double fault handler not called]");
    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mali_os::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
}
