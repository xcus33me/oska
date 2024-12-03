#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use vga_buffer::clear_vga_buffer;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("byi byi byi {} {}", "hella racks", "im finna breach");

    #[cfg(test)]
    test_main();

    loop {}
}

fn sleep(sec: u32) {
    let cycles_per_ms: u32 = 2000000;

    for _ in 0..sec * cycles_per_ms {
        unsafe {
            core::ptr::read_volatile(0x1000 as *const u8);
        }
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}