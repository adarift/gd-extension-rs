use retour::static_detour;
use std::error::Error;

static_detour! {
    static d_playlayer_update: unsafe extern "thiscall" fn(*const (), f32);
}


macro_rules! hook {
    ($d:ident -> $h:ident ($($n:ident: $t:ty),+) @ $off:expr => $hook:block) => {
        static_detour! {
            static $d: unsafe extern "thiscall" fn ($($t),+);
        }
        fn $h($($n: $t),+) {
            unsafe {
                $hook
            }
        }
        $d.initialize(std::mem::transmute($off),$h).unwrap().enable().unwrap();
    };
}

macro_rules! init_hook {
    ($d:ident -> $h:ident @ $off:expr) => {
        $d.initialize(std::mem::transmute($off),$h).unwrap()
            .enable().unwrap();
    };
}

pub fn get_base() -> usize {
    unsafe { winwrap::winapi::um::libloaderapi::GetModuleHandleA(0 as winwrap::winapi::um::winnt::LPCSTR) as usize }
}


pub(super) unsafe fn init()  -> Result<(), Box<dyn Error>> {
    let base = get_base();
    
    // Creating and initalizing hook using macro (more compact way of creating hook)
    hook!(d_playlayer_init -> h_playlayer_init (this: *const(), GJGameLevel: *const()) @ base + 0x1FB780 => {
        println!("Playlayer initalized");
        d_playlayer_init.call(this, GJGameLevel);
    });

    // Traditional way of creating a hook (see static_detour! macro at the beginning and h_playlayer_update function)
    init_hook!(d_playlayer_update -> h_playlayer_update @ base + 0x2029C0);
    
    Ok(())
}


fn h_playlayer_update(this: *const (), delta: f32) {
    unsafe {
        println!("Playlayer updated");
        d_playlayer_update.call(this, delta);
    }
}

