fn main() {
    println!("Hello, world!");
    println!("{}", get_grade(100, 90, 0))
}
fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let avg = (s1 + s2 + s3) / 3;
    match avg {
        90.. => 'A',
        80.. => 'B',
        70.. => 'C',
        60.. => 'D',
        _ => 'F',
    }
}
