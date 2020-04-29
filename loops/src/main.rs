fn main() {
    let mut count = 0;

    let _result = loop {
        count += 1;

        if count == 10 {
            println!("Done counting");
            break count * 2;
        }
    };

    println!("Value of count variable is {}", count);
    println!("Working with while loop now");

    let mut number: i32 = 10;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("Lift off!");
    println!("Working with an array:");

    let list_of_even_numbers: [i32; 5] = [2, 4, 6, 8, 10];
    loop_array(&list_of_even_numbers);

    println!("Using for loops instead:");

    for item in list_of_even_numbers.iter() {
        println!("The present value is {}", item);
    }
}

fn loop_array(array: &[i32]) {
    let mut count = 0;

    while count != array.len() {
        println!("Element {} of array is {}", count, array[count]);
        count += 1;
    }
} 
