#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn hello();
    }
}

pub fn hello() {
    println!("Hello, world!");
}
