#[macro_use]
extern crate stdweb;

mod salud;

#[no_mangle]
pub fn console_log(s: &str) {
    js!(console.log(@{s}););
}

pub use crate::salud::saludar;

