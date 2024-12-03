#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

use vga_buffer::print_some;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"HELLO WORLD!"; // 12 length
static NEW_WORLD: &[u8] = b"POPKA IS COMING FOR YOU LIL NIGGA I WILL TOUCH YOU";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sleep(10000); // 5 SECONDS

    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb1;
        }
    }

    sleep(10000);

    clear_vga_buffer();

    print_some();

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

pub fn clear_vga_buffer() {
    let vga_buffer = 0xb8000 as *mut u8;

    let num_chars = 80 * 25;

    for i in 0..num_chars {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = 0x00;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07;
        }
    }
}