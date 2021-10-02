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

INSString.as_str() internally uses UTF8String property of NSString. 
[Apple doc](https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string?language=objc)
 says that the memory behind this pointer has a lifetime shorter than a lifetime of an NSString itself. 
But apparently, this is not entirely true. At least, this statement is not valid for strings that contain 
characters outside the ASCI range. And sometimes for strings that do not.

Sadly, I did not find any reason for that.

So in the end, the actual leak occurs not in INSString.as_str() but, I guess, in objc runtime. 

## Is there a workaround?

Yes. NSString::getCString ([Apple doc](https://developer.apple.com/documentation/foundation/nsstring/1415702-getcstring)) and we can use it like this:

```rust
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
```
If you change in main.rs leak to no_leak_vec and will run again 
```shell
cargo instruments -t Allocations
```

You will see no leak at all. And if you run benchmark
```shell
cargo becnh
```
You will see that performance even better.

```shell
to string/old           time:   [12.855 us 12.961 us 13.071 us]                                                    
to string/new vec       time:   [10.477 us 10.586 us 10.699 us]   
```
 

The only problem I see with this solution is that it has a different return type (String instead of &str). 
If you know how to fix that, or any other idea on how to do things better - please let me know.  

## background

If you want to know how it all started - here is initial [blog post](https://barhamon.com/post/rust_and_nsstring) and [pull request to a copypasta](https://github.com/alacritty/copypasta/pull/33).