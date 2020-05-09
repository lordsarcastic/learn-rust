fn main() {
    let value = Some(3u8);

    match value {
        Some(3) => println!("Hello world"),
        _ => println!("Bye, world!"),
    };

    if let Some(3) = value {
        println!("Hello world");
    };
}
