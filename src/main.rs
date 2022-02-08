#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    "Welcome to McAuley OS!"
    .as_bytes()
    .iter()
    .flat_map(|bt| [*bt, 0xf as u8])
    .enumerate()
    .for_each(|(i, byte)| unsafe {
        *vga_buffer.offset(i as isize) = byte;
    });
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}