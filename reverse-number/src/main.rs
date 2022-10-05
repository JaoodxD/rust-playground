fn main() {
    println!("{:?}", digitize(12345))
}
fn digitize(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .rev()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}
