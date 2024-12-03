#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("byi byi byi {} {}", "hella racks", "im finna breach");

    loop {}
}

fn sleep(millis: u32) {
    let cycles_per_ms: u32 = 1000;

    for _ in 0..millis * cycles_per_ms {
        unsafe {
            core::ptr::read_volatile(0x1000 as *const u8);
        }
    }
}