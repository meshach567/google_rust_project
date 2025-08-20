fn fib(n: u32) -> u32 {
    if n < 2 {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 10;
    println!("The {}th Fibonacci number is: {}", n, fib(n))
}