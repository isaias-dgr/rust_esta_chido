// mod control_flow;
// mod ownership;
// mod references;
mod fundamental_types;

fn main() {
    // greeting();
    // greeting_name("Isaias");
    // let g = get_greeting_name("Daniel");
    // println!("{} 🌎", g);
    // control_flow::examples();
    // ownership::examples();
    // references::examples();\
    fundamental_types::examples();
}

fn greeting() {
    println!("Hello world! 🤝");
}

fn greeting_name(name: &str) {
    println!("Hello {}! 😎", name);
}

fn get_greeting_name(name: &str) -> String {
    return format!("Hello {}!", name).to_string();
}
