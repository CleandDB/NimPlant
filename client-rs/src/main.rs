// Hide the console window
#![windows_subsystem = "windows"]
// Enable features for COFF loading
#![allow(internal_features)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]

mod app;

fn main() {
    app::main();
}
