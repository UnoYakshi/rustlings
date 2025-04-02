const THRESHOLD: usize = 100;

fn main() {
    let a = [0; THRESHOLD + 1];

    if a.len() >= THRESHOLD {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
