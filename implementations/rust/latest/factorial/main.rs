fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    let n: u64 = 5; // Change this value to compute the factorial of a different number
    let result = factorial(n);
    println!("The factorial of {} is: {}", n, result);
}
