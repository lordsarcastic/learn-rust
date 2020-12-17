use std::fs::File;
use std::io::ErrorKind;


fn errors_with_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn errors_with_expect() {
    let f = File::open("hello.txt").expect("Failed to open file!");
}

fn errors_with_match() {
    let f = File::open("hello.txt");


    let _f = match f {
        Ok(file) => {
            print!("Nice bruh, nice!\n");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    print!("Neat bruh, neat!\n");
                    fc
                },
                Err(err) => panic!("Tried to create file but failed due to {:?}", err),
            },
            _other_error => panic!("Something evil happened!"),
        }
    };
}

fn errors_with_unwrap_or_else() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but failed due to {:?}", error);
            })
        } else {
            panic!("Something evil happened!");
        }
    });
}

fn main() {
   errors_with_unwrap();
   errors_with_expect();
   errors_with_match();
   errors_with_unwrap_or_else();
}
