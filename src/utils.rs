use std::io;

pub fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
