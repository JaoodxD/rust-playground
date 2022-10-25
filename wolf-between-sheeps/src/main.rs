fn main() {
    println!("{}", warn_the_sheep(&["sheep", "sheep", "wolf", "sheep"]));
}
fn warn_the_sheep(queue: &[&str]) -> String {
    match queue.iter().rev().position(|&s| s == "wolf").unwrap() {
        0 => "Pls go away and stop eating my sheep".to_string(),
        pos => format!("Oi! Sheep number {pos}! You are about to be eaten by a wolf!"),
    }
}
