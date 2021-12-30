fn main() {
    greeting();
    greeting_name("Isaias");
    let g = get_greeting_name("Daniel");
    println!("{} 🌎", g);
}

fn greeting() {
    println!("Hello world! 🖖");
}

fn greeting_name(name: &str) {
    println!("Hello {}! 😎", name);
}

fn get_greeting_name(name: &str) -> String {
    return format!("Hello {}!", name).to_string();
}
