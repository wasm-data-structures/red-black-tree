#![feature(lang_items, start)]
#![no_std]

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    0
}
