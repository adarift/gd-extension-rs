#[macro_export]
macro_rules! to_c_string {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as _
    };
}
#[macro_export]
macro_rules! from_c_string {
    ($s:literal) => {
        unsafe { CStr::from_ptr($s) }
    };
    ($s:ident) => {
        unsafe { CStr::from_ptr($s as *const i8) }
    };
    ($s:expr) => {
        unsafe { CStr::from_ptr($s as *const i8) }
    };
}
#[macro_export]
macro_rules! array {
    ($n:ident,+) => {
        
    };
    ($n:literal,+) => {
        
    };
}