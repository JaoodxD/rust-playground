fn main() {
    println!("Hello, world!");
    println!("{}", update_light("green"));
    println!("{}", update_light2("green"));
    println!("{}", update_light2("yellow"));
    println!("{}", update_light2("red"));


}

fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow",
        "yellow" => "red",
        "red" => "green",
        _ => "unreachable"
    }.to_string()
}

fn update_light2(current: &str) -> String {
    ["green", "yellow", "red"]
    .iter()
    .cycle()
    .skip_while(|&&x| x != current)
    .skip(1)
    .next()
    .unwrap()
    .to_string()
}
