use std::io;

fn convert_f_to_c(temp: f32) -> f32 {
    (temp - 32.0) / 1.8
}

fn convert_c_to_f(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}

fn main() {
    loop {
        println!("Here, we have a temperature converter");
        println!("Enter:");
        println!("'1', to convert from fahrenheit to celsius;");
        println!("'2', to convert from celsius to fahrenheit");

        let mut option = String::new();

        io::stdin().read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => {
                if (num != 1) && (num != 2) {
                    println!("Invalid option");
                    break;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("Invalid option");
                break;
            }
        };

        println!("Enter temperature value: ");

        let mut value = String::new();

        io::stdin().read_line(&mut value)
            .expect("Failed to read line");

        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if option  == 1 {
            println!("{}F in celsius is {}C", value, convert_f_to_c(value));
        } else if option == 2 {
            println!("{}C in fahrenheit is {}F", value, convert_c_to_f(value));
        }
    }
}
