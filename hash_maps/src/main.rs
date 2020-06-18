use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // initializing a HashMap
    let mut scores = HashMap::new();

    // inserting values into the HashMap
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);

    // creating HashMap using the zip and collect method from vector
    // of tuples
    let teams = vec![String::from("green"), String::from("yellow")];
    let points = vec![10, 20];

    let items: HashMap<_, _> = teams.iter().zip(points.iter()).collect();
}
