fn main() {
    let mut s = String::from("hello");

    s.push_str(", worls!");

    println!("{}", s);
    println!("Working with freeing of space");

    let s1 = String::from("Hello");
    let s2 = should_throw_error(s1);

    //println!("{} is s2 while {} is s1", s2, s1); //#will throw error if run

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{} is s2 while {} is s1", s2, s1);
    println!("Using functions and scopes");

    let s1 = gives_ownership();

    let s2 = takes_and_gives_back(s1.clone());

    println!("{} is s2 while {} is s1", s2, s1);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello world");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}

fn should_throw_error(s: String) -> String {
    s
}
