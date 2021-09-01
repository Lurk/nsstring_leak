use foundation::leak;

fn main() {
    for _ in 0..1_000_000 {
        leak("aaabbb");
    }
}
