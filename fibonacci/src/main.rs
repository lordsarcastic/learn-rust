use std::io;

fn fibonacci(nth: u16) {
    let mut count = 0;
    let mut initial = 0;
    let mut mid = 1;
    let mut present;

    if nth >= 1 {
         println!("Term 1 = {}", initial);
         count += 1;
    }
 
    if nth >= 2 {
        println!("Term 2 = {}", mid);
        count += 1;
    }


    while count < nth {
        count += 1;
        present = initial + mid;
        initial = mid;
        mid = present;
        println!("Term {} = {}", count, present);
    }
}

fn main() {
    loop {
        println!("Enter a nth term to generate fibonacci sequence:");

        let mut term = String::new();

        io::stdin().read_line(&mut term)
            .expect("Failed to read line");

        let term: u16 = match term.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                break;
            },
        };

        fibonacci(term);
    }
}
