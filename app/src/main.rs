fn main() {
    test_func();
}
fn test_func() {
    let ages: [u16; 5] = [61, 62, 63, 64, 65];

    println!("{:?}", ages);

    let new_ages: &[u16] = &ages[1..4];
    println!("{:?}", new_ages);
}
