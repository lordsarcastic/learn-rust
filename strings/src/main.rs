fn main() {
    println!("Hello, world!");

    // initializing strings
    let s1 = String::new();

    // converting string literal into string object/struct?
    let data = "Initial contents";
    let mut s2 = data.to_string();

    // working with string updating/appending
    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);

    // push is for single characters
    s3.push('s');
    println!("s4 is {}", s3);

    // string concatenation. we do it this way since the add is a
    // method called upon s1 with s2 as a &String argument. s1 is no
    // longer valid after here
    let s5 = s3 + &s4;

    // concatenation for multiple strings
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    println!("tic tac toe is {}", tic_tac_toe);
    
    // concatenation using format macro. for this to work, previous
    // snippet last two lines has to be disabled because of ownership 
    // issues
    // let tic_tac_toe = format!("{}-{}-{}", &tic, &tac, &toe);
    // println!("tic tac toe is {}", tic_tac_toe);

    // handling string indexing
    let hello = "Здравствуйте";

    // working with chars. behaviour is not same across languages
    for ch in hello.chars() {
        println!("{}", ch);
    }

    // working with bytes. behaviour is not also constant across languages
    for ch in hello.bytes() {
        println!("{}", ch);
    }
}
