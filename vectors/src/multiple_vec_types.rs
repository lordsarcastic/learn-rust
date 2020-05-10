enum VecType {
    Int(i32),
    Float(i64),
    Text(String),
}

fn main() {
    let values = vec![VecType::Int(3), VecType::Float(34.3), VecType::Text(String::from("Hello"))];
}
