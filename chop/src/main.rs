use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    let mut input1 = String::new();
    println!("Enter the percent success rate a number between 0 and 1");
    io::stdin().read_line(&mut input1).expect("Bad 1");
    let val1: f32 = input1.trim().parse().expect("Bad 1.1");

    let mut input2 = String::new();
    println!("Enter the times it is tried E.G. 10");
    io::stdin().read_line(&mut input2).expect("Bad 2");
    let val2: i32 = input2.trim().parse().expect("Bad 2.1");

    println!("Calculating...");

    let mut count_it = 0;
    for _ in 0..1_000 {
        let mut finger_there = true;
        for _ in 0..val2 {
            let my_number = rng.gen::<f32>();
            if my_number >= val1 { finger_there = false; }
        }
        if finger_there == true { count_it += 1; }
    }
    println!("You keep your finger {count_it} times out of 1_000");
}
