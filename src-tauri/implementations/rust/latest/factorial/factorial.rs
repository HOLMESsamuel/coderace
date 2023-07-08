pub fn factorial(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return n*factorial(n-1);
    }
}