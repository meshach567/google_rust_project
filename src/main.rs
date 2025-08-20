fn main() {
    // let mut x = 100;
    // while  x >= 10 {
    //     x = x / 2;
    // }
    // dbg!(x);
    let mut x = 0;
    loop {
        x += 1;
        if x > 100 {
            break;
        }
        
        if x % 2 == 0 {
            continue;
        }
        dbg!(x);
    }
}