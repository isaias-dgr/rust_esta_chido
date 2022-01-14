use std::f32;

pub fn examples() {
    println!("\nFundamental Types 😎");
    // inferencia de tipos
    let v = build_vector();
    println!("Vector {:?}", v);
    // Type primitives
    example_integers();
    example_float();

}

fn example_float(){
    let num = 3.0;
    let num = 2.3323_f32;
    // let num: f32 = .343;
    let num: f32 = 3.4;
    let num: f64 = 23.3223234234;
    println!("Pi {} {}", f32::consts::PI, f32::consts::E);
    println!("3 / 2 = {}", 3/2);
    println!("3 / 2 = {}", 3 as f32 /2 as f32);
    println!("3 / 2 = {}", 3.0/2.0);
    let num = 3.0_f32/2.2;
    println!("3 / 2 = {:.3} ≅ {} ≅ {}", num, num, num.round());
    let i = num as i16; // i32
    println!("3 / 2  ≅ {}", i);
    
}


fn example_integers(){
    let num = 1;
    let num: u8 = 2;
    let num1 = 3u8;
    let num2 = 3u16;
    // let comp =  num1 == num2;
    let comp = num1 == num2 as u8;
    let num = 3_002;
    let num_bin = 0b100010;
    let num_hex = 0xffff;
    let num_oct = 0o7777;
    println!(
        "Numero {} {:0b} {:0X} {:0o} ",
        num, num_bin, num_hex, num_oct
    );

    assert_eq!(2_u16.pow(9), 512_u16);

    // let mut i = 1;
    // loop {
    //     i *= 10;
    //     println!("-> {}", i);
    // }

    // let mut i: u8 = 1;
    // loop {
    //     i = i.checked_mul(10).expect("🙀 multiplication overflow");
    // }
    // println!("-> {}", i);
    assert_eq!(10_u16.wrapping_mul(2433), 24330);
    assert_eq!(10_u8.wrapping_mul(243), 126);
    assert_eq!(18_u16.wrapping_mul(22243), 7158);

    assert_eq!(10_u8.saturating_mul(243), 255);
    assert_eq!(-10_i8.saturating_mul(127), -127);
    assert_eq!(10_u8.overflowing_mul(243), (126, true));
    assert_eq!(18_u16.overflowing_mul(22243), (7158, true));
}
// inferencias de tipos
// fn build_vector() -> Vec<i16> {
//     let mut v: Vec<i16> = Vec::<i16>::new();
//     v.push(10i16);
//     v.push(20i16);
//     v
// }

// inferencias de tipos
fn build_vector() -> Vec<i16> {
    let mut v = Vec::<i16>::new();
    v.push(10);
    v.push(20);
    v
}
