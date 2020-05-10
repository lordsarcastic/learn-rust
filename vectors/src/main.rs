fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new();

    let mut v1 = vec![1, 2, 3];
    v1.push(4);
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    //multiple methods of accessing vector values
    let third = &v1[2];

    match v1.get(2) {
        Some(third) => println!("The third element of the vector is {}", third),
        None => println!("There is no third element"),
    };

    //from both types of calls above, the first will stop program execution, while the other will
    //return an Option<T> of None. This means I can choose path of my program behaviour.

    v1.push(9);
    //this will throw an error because of mutable and immutable references above using the
    //```third``` variable.
    //println!("{}", third);

    for i in &mut v1 {
        println!("{}", i);
        *i += 50;
        println!("i is now {}", i);
    }
}
