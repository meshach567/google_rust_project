fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n: u32 = 20;
    match n {
        10 => println!("fib({n}) = {}", fib(n)),
        20 => println!("fib({n}) = {}", fib(n)),
        _ => println!("fib({n}) = {}", fib(n))
    }
}