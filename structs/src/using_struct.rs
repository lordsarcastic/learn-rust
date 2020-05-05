#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
    //self can only hold rect if its width and height is greater than width and height of rect

        if (self.width > rect.width) && (self.height > rect.height) {
            return true;
        }

        false
    }

    fn make_square(dim: i32) -> Rectangle {
        Rectangle { width: dim, height: dim }
    }

    fn show_details(&self) {
        println!("Shape has width: {} and height: {}", self.width, self.height);
    }
}

fn main() {
    let rect1 = Rectangle { width: 32, height: 36 };
    let rect2 = Rectangle { width: 14, height: 12 };
    let rect3 = Rectangle::make_square(4);

    rect1.show_details();
    rect2.show_details();
    rect3.show_details();

    /*println!(
        "The area of rectangle with width: {} and height {} is {}",
        rect1.width, rect1.height, rect1.area()
    );*/

    //println!("This is the rect1 variable {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));
}
