#![allow(clippy::ptr_arg)]

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();
    get_char(&data);
    string_uppercase(data);

    let mut s4 = String::from("hello");
    let r1 = &s4;
    let r2 = &s4;
    println!("{} and {}", r1, r2);
}
