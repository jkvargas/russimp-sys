#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl From<aiString> for String {
    fn from(string: aiString) -> Self {
        unsafe {
            std::str::from_utf8(std::slice::from_raw_parts(
                string.data.as_ptr() as *const u8,
                string.length as _,
            ))
        }
        .unwrap()
        .into()
    }
}

impl From<&aiString> for String {
    fn from(string: &aiString) -> Self {
        unsafe {
            std::str::from_utf8(std::slice::from_raw_parts(
                string.data.as_ptr() as *const u8,
                string.length as _,
            ))
        }
        .unwrap()
        .into()
    }
}
