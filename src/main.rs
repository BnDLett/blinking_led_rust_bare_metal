#![no_std]
#![no_main]

//3F20_0008 fsel2 1<<3 turn pin21 into out
//3F20_001c gpio1 set 1<<21 turns on pin21
//3F20_0028 gpio1 set 1<<21 turns off pin21

use core::{panic::PanicInfo, ptr};
use core::arch::asm;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // turn pin 21 into an output
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);

        loop {
            // Turn pin on
            core::ptr::write_volatile(0x3F20_001c as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop");
            }

            // Turn pin off
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<21);
        }
    }
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop {}
}
