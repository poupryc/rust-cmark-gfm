#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn check_version() {
        let version_macro = CStr::from_bytes_with_nul(CMARK_GFM_VERSION_STRING).unwrap();
        let version_fn = unsafe { CStr::from_ptr(cmark_version_string()) };

        assert_eq!(version_fn, version_macro);
    }
}
