fn main() {
    println!("Hello, world!");

    let s = String::from("hello world, how are you");

    let index = second_word_slice(&s);

    println!("The second word is '{}'", index);
   //println!("the index of the second word is {} and {}", index[0], index[1]);

    let a = [1, 2, 3, 4, 5];
    let b = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let word = s.as_bytes();

    for (i, &item) in word.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn second_word(s: &String) -> [usize; 2] {
    let word = s.as_bytes();
    let mut loc:[usize; 2] = [s.len(), s.len()];
    let mut found = false;

    for (i, &item) in word.iter().enumerate() {
        if (item == b' ') && (found == false) {
            loc[0] = i;
            found = true;
            continue;
        } else if (item == b' ') && (found == true) {
            loc[1] = i;
            return loc;
        }
    }

    loc
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word_slice(s: &str) -> &str {
    let word = s.as_bytes();
    let mut loc = s.len();
    let mut found = false;

    for (i, &item) in word.iter().enumerate() {
        if (item == b' ') && (found == false) {
            loc = i;
            found = true;
            continue;
        } else if (item == b' ') && (found == true) {
            return &s[loc+1..i];
        }
    }

    &s
}

