fn interproduct(a: i16, b: i16, c: i16) -> i16 {
    return (a * b).saturating_add(b * c).saturating_add(c * a);
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
}
