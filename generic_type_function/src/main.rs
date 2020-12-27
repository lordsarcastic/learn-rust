fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let i32_list = vec![28, 84, 74, 55, 30];
    let char_list = vec!['x', 'o', 'b', 'y'];

    println!("The largest number is {}", largest_i32(&i32_list));
    println!("The largest char is {}", largest_char(&char_list));
    println!("The largest char is {}", generic_largest(&char_list));
}
