#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
2fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

