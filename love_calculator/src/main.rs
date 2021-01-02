use std::cmp::Ordering;
use std::convert::TryInto;
use std::io;


fn get_input(var: &mut String) -> i32 {
    io::stdin()
        .read_line(var)
        .expect("Failed to read line");

    var.chars().count().try_into().expect("Length of name is too large!")
}

fn calculate_compatibility(male_count: i32, female_count: i32) -> f32 {
    match male_count.cmp(&female_count) {
        Ordering::Less => {
            ((male_count as f32 / female_count as f32) as f32 * 100.0) as f32
        },
        Ordering::Greater => {
            ((female_count as f32 / male_count as f32) as f32 * 100.0) as f32
        },
        Ordering::Equal => {
            50.0
        }
    }
}


fn main() {
    let mut male = String::new();
    let mut female = String::new();

    println!("Enter male name: ");
    let male_count = get_input(&mut male);

    println!("Enter female name: ");
    let female_count = get_input(&mut female);

    let compat_value = calculate_compatibility(male_count, female_count).floor();
    
    println!("Love compatibility between {} and {} is {}%", male.trim(), female.trim(), compat_value);
}
