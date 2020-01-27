#![no_std]
#![feature(lang_items)]
use core::panic::PanicInfo;
mod sys;

#[no_mangle]
pub extern "C" fn crawler() {
    sys::video::init_video();
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
