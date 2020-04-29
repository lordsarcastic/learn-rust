fn main() {
    println!("Hello, world!");
    
    let x = 5;

    if x > 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let y = if x > 5 {
        10
    } else {
        0
    };
    println!("Y is {}", y);
}
