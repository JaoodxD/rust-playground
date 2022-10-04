fn main() {
    println!("Hello, world!");
    let str = "123";
    println!("{}", string_to_number(str));
}
fn string_to_number(s: &str) -> i32 {
    s.parse().expect("NaN")
}
