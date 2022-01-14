pub fn examples() {
    println!("\nOwnership 🙀");
    scope_mut();
    scope_func();
}

fn scope_func(){
    println!("\nScope/Ownership in functions");
    let text = String::from("Hi!!");
    let num: u8 = 136;
    take_ownership(text);
    // println!("Text: {}", text);
    take_copy(num);
    println!("Num: {}", num);   
    
    let text = String::from("Hi!!");
    let new_text = return_ownership(text);
    println!("Text: {}", new_text);
}

fn take_ownership(some_string: String){
    println!("Text: {}", some_string);
}

fn return_ownership(mut some_string: String) -> String {
    some_string.push_str("return_ownership Hola!");
    println!("Text: {}", some_string);
    some_string
}

fn take_copy(arg: u8){
    println!("Num: {}", arg);
}

fn scope_mut(){
    println!("\nScope/Ownership");
    let s = "Hi";
    let mut x: u16 = 10;
    {
        // let s = "Hello";
        // let mut x: u8 = 40;
        x += 2;
        println!("In a block Var s = {}, x = {}", s, x);
    }
    x+=1626;
    println!("Out of a block Var s = {}, x = {}", s, x);

    {
        let s = String::from("hello");
        println!("s = {}", s);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1; // move owner
        let s3 = s2.clone();
        println!("s1 = {} s2 = {}", s, s2);
        println!("s1 = {} s2 = {}", s3, s2);
    }
}