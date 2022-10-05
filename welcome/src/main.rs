fn main() {
    println!("{}",greet("some non-existing stuff"));
}
use std::collections::HashMap;
fn greet(language: &str) -> &str {
    let dictionary = HashMap::from([
        ("english", "Welcome"),
        ("czech", "Vitejte"),
        ("danish", "Velkomst"),
        ("dutch", "Welkom"),
        ("estonian", "Tere tulemast"),
        ("finnish", "Tervetuloa"),
        ("flemish", "Welgekomen"),
        ("french", "Bienvenue"),
        ("german", "Willkommen"),
        ("irish", "Failte"),
        ("italian", "Benvenuto"),
        ("latvian", "Gaidits"),
        ("lithuanian", "Laukiamas"),
        ("polish", "Witamy"),
        ("spanish", "Bienvenido"),
        ("swedish", "Valkommen"),
        ("welsh", "Croeso"),
    ]);
    match dictionary.get(language) {
        Some(message) => message,
        None => dictionary.get("english").unwrap(),
    }
}
