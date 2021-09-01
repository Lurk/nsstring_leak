use objc_foundation::{NSString, INSString};
use objc::{msg_send, sel, sel_impl};
use std::ffi::{CString, c_void};
use objc_id::Id;
use std::os::raw::c_char;

pub fn leak(str: &str) {
    let nsstrig = NSString::from_str(str);
    nsstrig.as_str();
}

pub fn no_leak(str: &str) {
    let nsstrig = NSString::from_str(str);
    nsstring_to_rust_string(nsstrig);
}


/**
Function that converts NSString to rust string through CString to prevent a memory leak.

encoding:
   4 = NSUTF8StringEncoding
   https://developer.apple.com/documentation/foundation/1497293-string_encodings/nsutf8stringencoding?language=objc

getCString:
   Converts the string to a given encoding and stores it in a buffer.
   https://developer.apple.com/documentation/foundation/nsstring/1415702-getcstring

 */
pub fn nsstring_to_rust_string(nsstring: Id<NSString>) -> String {
    unsafe {
        let string_size: usize = msg_send![nsstring, lengthOfBytesUsingEncoding: 4];
        // + 1 is because getCString returns null terminated string
        let buffer = libc::malloc(string_size + 1) as *mut c_char;
        let is_success: bool = msg_send![nsstring, getCString:buffer  maxLength:string_size+1 encoding:4];
        if is_success {
            // CString will take care of memory from now on
            CString::from_raw(buffer).to_str().unwrap().to_owned()
        } else {
            // In case getCString failed there is no point in creating CString
            // So we must free memory
            libc::free(buffer as *mut c_void);
            // Original NSString::as_str() swallows all the errors.
            // Not sure if that is the correct approach, but we also don`t have errors here.
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nsstrig = NSString::from_str("aaabbb🍺Ыض");
        assert_eq!(nsstring_to_rust_string(nsstrig), "aaabbb🍺Ыض");
    }
}