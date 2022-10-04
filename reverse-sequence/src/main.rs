fn main() {
    println!("Hello, world!");
    println!("Reversed sequence: {:?}", reverse_seq(5))
}
fn reverse_seq(n: u32) -> Vec<u32> {
    (1..=n).rev().collect()
}
