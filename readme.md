# This repo is about reproducible memory leak in objc_foundation`s INSstring.as_str().



## how to see a leak

```shell
xcode-select --install
brew install cargo-instruments  
cargo instruments -t Allocations
```

## what produces a leak?

```rust
pub fn leak(str: &str) {
    let nsstrig = NSString::from_str(str);
    nsstrig.as_str(); // if you replace just this one line leak is gone
}
```

## why it produces a leak?

INSString.as_str() internaly uses UTF8String property of NSString. 
[Apple doc](https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string?language=objc)
 says that the memory behind this pointer has a lifetime shorter than a lifetime of an NSString itself. 
But apparently, this is not entirely true. At least, this statement is not valid from strings that contain 
characters outside the ASCI range. And sometimes for strings that do not.

Sadly, I did not find any reason for that.

So in the end, the actual leak occurs not in INSString.as_str() but, I guess, in objc runtime. 

## Is there a warkaround?

my proposal is:

```rust
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
```
If you change in main.rs leak to no_leak and again will run again 
```shell
cargo instruments -t Allocations
```

You will see no leak at all. And if you run benchmark
```shell
cargo becnh
```
You will see that performance even better.


The only problem I see with this solution is that it has a different return type (String instead of &str). 
If you know how to fix that, or any other idea on how to do things better - please let me know.  

## background

If you want to know how it all started - here is initial [blog post](https://barhamon.com/post/rust_and_nsstring) and [pull request to a copypasta](https://github.com/alacritty/copypasta/pull/33).