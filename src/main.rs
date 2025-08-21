// fn fib(n: u32) -> u32 {
//     if n < 2 {
//         return n
//     } else {
//         return fib(n - 1) + fib(n - 2)
//     }
// }

fn collatz(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        len += 1;
    }
    len
}

fn main() {
   println!("Length: {}", collatz(11));
}