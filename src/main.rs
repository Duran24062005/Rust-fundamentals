/* fn main() {
    let texto_multilinea = r#"
    Esta es la primera línea.
    Esta es la segunda línea.
    Y esta es la tercera.
    "#;
    println!("{}", texto_multilinea);
    println!(" ===== Hello, world! =====");
    println!(r"< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \");
}*/


use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}