fn main() {
    println!("Hello, world!");
    let x = five();
    println!("Value of x is {}", x);   
}

fn show_value(x: i32) {
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}
