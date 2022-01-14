pub fn examples() {
    println!("\nControl flow 🙀")
    max_min_eq(20, 20);
    max_min_eq(32, 20);
    max_min_eq(32, 200);
    let max = get_max(32, 20);
    println!("🦀 Max: {} ", max);
    simple_loop(10);
    while_loop(10, 0);
    for_loop();
}

fn max_min_eq(num1: u8, num2: u8) {
    if num1 == num2 {
        println!("🦀 {} and {} are equal", num1, num2);
    } else if num1 > num2 {
        println!("🦀 {} is greater than {}", num1, num2);
    } else {
        println!("🦀 {} is greater than {}", num2, num1);
    }
}

fn get_max(num1: u8, num2: u8) -> u8 {
    let max = if num1 > num2 { num1 } else { num2 };
    return max;
}

fn simple_loop(stop: u8) {
    let mut count: u8 = 0;
    loop {
        if count == stop {
            break;
        }
        count += 1;
        println!("🦀 Loooooop! {} 🎡", count);
    }
}

fn while_loop(mut start: u8, stop: u8) {
    while start != stop {
        println!("🦀 Whiiile! {} 🎡", start);
        start -= 1;
    }
}

fn for_loop() {
    for number in (1..4).rev() {
        println!("🦀 fooor {}! 🎡", number);
    }
    println!("LIFTOFF!!!");
}
