use foundation::no_leak_autoreleasepool;

fn main() {
    for _ in 0..1_000_000 {
        no_leak_autoreleasepool("aaabbb");
    }
}
