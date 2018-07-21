use std::ffi::CString;

pub fn string_cast(s: &str) -> *const i8 {
    let s = CString::new(s).unwrap();
    s.as_ptr()
}

pub fn string_cast_mut(s: &str) -> *mut i8 {
    CString::new(s).unwrap().into_raw()
}

pub fn ram_to_string(raw: *mut i8) -> String {
    unsafe { CString::from_raw(raw).into_string().unwrap() }
}
