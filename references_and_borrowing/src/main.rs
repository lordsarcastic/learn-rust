fn main() {
    let mut value = String::from("Hello world");

    let len = calculate_length(&value);

    change(&mut value);

    println!("The length of '{}' is {}", value, len);

    println!("Working with mutable and immutable references");

    let r1 = &value;
    let r2 = &value;

    //println!("r1 is {}, r2 is {}", r1, r2);
    let r3 = &mut value;
    println!("r3 is {}", r3);
}

fn calculate_length(string: &String) -> usize {
    string.len()
}

fn change(string: &mut String) {
    string.push_str(", this is lord_sarcastic");
}
