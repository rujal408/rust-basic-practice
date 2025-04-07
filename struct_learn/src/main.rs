#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("Hello"),
        email: String::from("r@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;

    println!("{}", name);

    user1.username = String::from("Hello Updated");

    let user2 = build_user(String::from("k@test.com"), String::from("value"));

    let user3 = User {
        username: String::from("ABC"),
        email: String::from("av@test.com"),
        ..user2 // Like a spread operator
    };

    println!("{:#?}", user3);

    println!("The area is {}", area((12, 2)));

    let rect = Rectangle {
        height: 22,
        width: 10,
    };

    println!("The area of rectangle is {}", rectangle_area(&rect));

    struct Square {
        size: u32,
    }

    impl Square {
        fn area(&self) -> u32 {
            self.size * self.size
        }

        fn can_hold(&self, other: &Square) -> bool {
            self.size > other.size
        }
    }

    let square = Square { size: 2 };
    println!("Area of square is {}", square.area());

    let sq1 = Square { size: 2 };

    let sq2 = Square { size: 1 };

    println!("{}", sq1.can_hold(&sq2));

    let square_from_rectangle = Rectangle::square(3);
    println!(
        "The square from rectangle is is {:#?}",
        square_from_rectangle
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 2,
        active: false,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
