fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    
    let c = 'c';
    println!("c is {}", c);
    let tup: (i8, f32, u16) = (100, 4.16, 200);
    let (x, y, z) = tup;
    
    let one_hundred = tup.0;

    let four_point_sixteen = tup.1;

    let  two_hundred = tup.2;

    println!("The values of the tuple are: {}, {} and {}", one_hundred, four_point_sixteen, two_hundred);
    
    let array = [32; 5];

    println!("Array values are: {} and {}", array[0], array[1]);
}
