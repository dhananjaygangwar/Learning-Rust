fn main() {
    test_func();
}
fn test_func() {
    let x: f32 = 255.0;
    let y: u8 = x as u8 - 5;
    println!("{:?}", y);
}
