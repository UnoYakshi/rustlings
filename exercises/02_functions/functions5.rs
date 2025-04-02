fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let input = 3;
    let answer = square(input);
    println!("The square of {input} is {answer}");
}
