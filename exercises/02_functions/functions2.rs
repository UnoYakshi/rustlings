use std::thread::{sleep, spawn};
use std::time::{Duration, SystemTime, Instant};


fn timeit<F: Fn() -> T, T>(f: F) -> T {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("It took {} milisecond(s).", duration.as_millis());
    result
}


/// Enumerated and multithreaded println...
fn call_me(lines_num: i64, threads_num: u32) {

    // TODO: Calculate threads number if not defined...

    // TODO: Run multiple threads...
    let handle = spawn(move || {
        for i in 1..lines_num {
            let x = i * i;
            println!("THREAD: {i} ==> {x}");
        }
    });
    handle.join().unwrap();

    println!("{threads_num} threads ran {lines_num} lines");
}

fn main() {
    timeit(|| call_me(1_000, 4));
}
