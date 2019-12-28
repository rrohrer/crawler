#![no_std]
#![feature(lang_items)]
use core::panic::PanicInfo;
use rs_nds;

macro_rules! c_str {
    ($s:expr) => {{
        concat!($s, "\0").as_ptr()
    }};
}

#[no_mangle]
pub extern "C" fn crawler() {
    unsafe {
        rs_nds::consoleDemoInit();
        let hello = c_str!("Hello from rust!");
        rs_nds::iprintf(hello);

        loop {}
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
