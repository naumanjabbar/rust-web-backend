use std::{thread, time};

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return  2
}

fn main() {
    let now = time::Instant::now();
    let one = do_something(1);
    let two = do_something(2);
    let three = do_something(3);

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", one + two + three);
}