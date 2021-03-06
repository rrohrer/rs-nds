# rs-nds
Bindings generated for libnds as provided by Devkitpro. They allow you to write rust code that can interact with the system APIs on a Nintendo DS. This is great for using rust for nds homebrew.

This depends on the toolchains made by DEVKITPRO, which needs to be installed and configured before using this library.

# Usage:
First make sure this is in your `Cargo.toml`:
```toml
[dependencies]
rs-nds = {git = "https://github.com/rrohrer/rs-nds"}

[lib]
name = "rnds"
crate-type = ["staticlib"]
```
Next, make a simple rust library that contains this hello-world:
```rust
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
pub extern "C" fn rnds() {
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
```
