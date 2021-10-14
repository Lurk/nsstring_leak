use objc::rc::autoreleasepool;
use objc::{msg_send, sel, sel_impl};
use objc_foundation::{INSString, NSString};
use objc_id::Id;
use std::ffi::CString;

pub fn leak(str: &str) {
    let nsstring = NSString::from_str(str);
    nsstring.as_str();
}

pub fn no_leak_autoreleasepool(str: &str) {
    let nsstring = NSString::from_str(str);
    convert_with_autoreleasepool(nsstring);
}

pub fn no_leak_vec(str: &str) {
    let nsstring = NSString::from_str(str);
    convert_with_vec(nsstring);
}

pub fn convert_with_autoreleasepool(nsstring: Id<NSString>) -> String {
    autoreleasepool(move || nsstring.as_str().to_owned())
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
pub fn convert_with_vec(nsstring: Id<NSString>) -> String {
    let string_size: usize = unsafe { msg_send![nsstring, lengthOfBytesUsingEncoding: 4] };
    let mut buffer: Vec<u8> = vec![0_u8; string_size + 1];
    let is_success: bool = unsafe {
        msg_send![nsstring, getCString:buffer.as_mut_ptr()  maxLength:string_size+1 encoding:4]
    };
    if is_success {
        // before from_vec_with_nul can be used https://github.com/rust-lang/rust/pull/89292
        // nul termination from the buffer should be removed by hands
        buffer.pop();

        unsafe {
            CString::from_vec_unchecked(buffer)
                .to_str()
                .unwrap()
                .to_string()
        }
    } else {
        // In case getCString failed there is no point in creating CString
        // Original NSString::as_str() swallows all the errors.
        // Not sure if that is the correct approach, but we also don`t have errors here.
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "aaabbb🍺Ыض";
        assert_eq!(convert_with_vec(NSString::from_str(text)), text);
        assert_eq!(convert_with_autoreleasepool(NSString::from_str(text)), text)
    }
}
