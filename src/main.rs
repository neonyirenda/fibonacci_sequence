use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("Enter Number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();
    println!("Fib {}: {}", number, fib(number));
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}