pub fn examples() {
    println!("\nReference Ownership 🙀");
     
    let s1 = String::from("Isaias");
    let len = calculate_length(&s1);
    calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    let mut s1 = String::from("Simple text");
    change(&mut s1);
    println!("text: {}", s1);

    let s1 = String::from("Isaias Daniel");
    let first = first_word(&s1);
    println!("The first word {}", first);
    let first = &s1[0..6];
    let second = &s1[7..13];
    println!("The first word {:?} {} {}", &s1[..1].to_lowercase(), first, second);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String){
    s.push_str(" More text");
}