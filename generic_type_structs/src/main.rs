struct UniformPoint<T> {
    x: T,
    y: T,
}

struct VariedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> UniformPoint<T> {
    fn x(&self) -> {
        &self.x
    }
}

fn main() {
    let integer = UniformPoint { x: 5, y: 10 };
    let float = UniformPoint { x: 3.5, y: 20.4 };
    //let mixed_type = UniformPoint { x: 3.4, y: 3 };

    let both_integer = VariedPoint{ x: 5, y: 10 };
    let both_float = VariedPoint { x: 3.5, y: 20.4 };
    let integer_and_float = VariedPoint{ x: 3.4, y: 3 };
}
