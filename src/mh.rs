use std::{thread, fs, ptr, ffi::CStr};
use winwrap::{winapi::shared::ntdef::LPCSTR, raw::um::libloaderapi::FreeLibraryAndExitThread};
use paste::paste;
use crate::{extension, to_c_string, from_c_string};


macro_rules! callback {
    ($callback_name:ident | $callback:block) => {
        extern "stdcall" fn $callback_name(ext: *mut ()) {
            $callback
        }
    };
    ($callback_name:ident & $ext_name:ident | $callback:block) => {
        extern "stdcall" fn $callback_name($ext_name: *mut ()) {
            $callback
        }
    };
    ($callback_name:ident @ $($n:ident: $t:ty),+ | $callback:block) => {
        extern "stdcall" fn $callback_name(ext: *mut (), $($n: $t),+) {
            $callback
        }
    };
    ($callback_name:ident & $ext_name:ident @ $($n:ident: $t:ty),+ | $callback:block) => {
        extern "stdcall" fn $callback_name(ext: *mut (), $($n: $t),+) {
            $callback
        }
    };
}

callback!(button_callback | {
    println!("Test button says hello!");
});

callback!(checked_callback | {
    println!("Checkbox checked!");
});

callback!(unchecked_callback | {
    println!("Checkbox unchecked!");
});

callback!(textbox_callback & ext | {
    println!("Textbox {:?}!", from_c_string!(extension::get_textbox_text(ext)));
});

callback!(combobox_callback @ option_number: i32, name_of_option: *const u8 | {
    println!("Combobox {:?}:{:?}!", from_c_string!(name_of_option), option_number);
});



pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let ext = extension::initialise_ext(to_c_string!("Extension"));
    
    // Rendering is going in reverse order

    let combo = extension::add_combobox(ext, combobox_callback);
    extension::set_combobox_strs(combo, [to_c_string!("Combobox"), to_c_string!("Combobox 2"), ptr::null()].as_mut_ptr());
    extension::set_combobox_index(combo, 0);

    let text = extension::add_textbox(ext, textbox_callback);
    extension::set_textbox_placeholder(text, to_c_string!("Textbox placeholder"));
    extension::set_textbox_text(text, to_c_string!("Textbox"));

    extension::add_checkbox(ext, to_c_string!("Checkbox"), checked_callback, unchecked_callback);

    extension::add_button(ext, to_c_string!("Button"), button_callback);
    
    extension::commit_ext(ext);

    Ok(())
}