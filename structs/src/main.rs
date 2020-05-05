struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn main() {
    fn build_user(username:String, email:String) -> User {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true,
        }
    }


    let mut user1 = User {
        username: String::from("Lord_Sarcastic"),
        email: String::from("adeoti.15.jude@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    //let mut user2 = build_user(String::from("Hello"), String::from("world"));

    //println!("{}", user2.username);

    println!("Creating instances from other instances: Normal method");

    let user2 = User {
        username: String::from("Chuck"),
        email: String::from("chuck@gmail.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let user3 = User {
        username: String::from("Rufus"),
        email: String::from("rufus@bonychicken.com"),
        ..user1
    };

    println!("Working with tuple structs");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
