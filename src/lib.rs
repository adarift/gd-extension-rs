#![feature(panic_info_message, abi_thiscall)]
#![allow(unused_variables)]

mod utils;
mod panic_hook;
mod hooks;
mod extension;
mod mh;


use winwrap::winapi;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::wincon::SetConsoleTitleA;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winwrap::winapi::shared::minwindef::*;
use winwrap::winapi::shared::minwindef::{FALSE, TRUE};

pub use winwrap::winapi as __getfn_winapi__;

// Main entry point
#[no_mangle]
pub extern "system" fn DllMain(dll: usize, reason: u32, _reserved: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(move || unsafe { main_thread(dll) });
    }
    TRUE
}

unsafe extern "system" fn main_thread(_hinst: usize) {
    AllocConsole(); // Allocating console for debug purposes
    SetConsoleTitleA(to_c_string!("Debug console"));
    panic_hook::init().unwrap(); // Inits panic hook. Refer to panic_hook.rs file
    mh::init().unwrap(); // Inits MHv6 extension window. Refer to mh.rs file.
    hooks::init().unwrap(); // Inits hooks. Refer to hooks.rs file
}