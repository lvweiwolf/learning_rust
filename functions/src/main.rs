fn main() {
    println!("Hello, world!");
    let x = another_function();
    println!("Function return: {x}");

    let s = String::from("Hello");
    let r = hold_reference(&s);

    println!("Return reference: {r}");
}

fn another_function() -> i32 {
    println!("Another function.");
    return 5;
}

fn hold_reference(s : &String) -> &String {
    let len = s.len();
    println!("String length is: {len}");
    return s;
}
