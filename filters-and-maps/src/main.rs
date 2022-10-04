fn main() {
    for c in "HeLLO BUddy".chars()
    .filter(|c| c.is_lowercase())
    .map(|c| c.to_uppercase()){
        print!("{}", c);
    }
}
