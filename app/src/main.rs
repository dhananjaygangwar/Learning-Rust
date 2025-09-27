pub mod helpers;

fn main() {
    // test_func();

    let myresult: String = helpers::namehelpers::get_full_name("djay", "jones");
    println!("Hello from {0}", myresult);

    let new_age = helpers::privatefns::get_age_plus_5(23);
    println!("new plus age is {0}", new_age);
}

#[allow(dead_code)]
fn test_func() {
    let ages: [u16; 5] = [61, 62, 63, 64, 65];

    println!("{:?}", ages);

    let new_ages: &[u16] = &ages[1..4];
    println!("{:?}", new_ages);
}
