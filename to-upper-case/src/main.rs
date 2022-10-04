fn main() {
    println!("{}", make_upper_case("Hello, world!"))
}
fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}
