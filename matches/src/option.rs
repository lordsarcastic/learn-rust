fn plus_one(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let num = Some(5);
    let six = plus_one(num);
    let nuthin = plus_one(None);
}
