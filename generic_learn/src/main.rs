#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x_val(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y_val(&self) -> f64 {
        self.y
    }
}

#[derive(Debug)]
struct Coor<T, U> {
    x: T,
    y: U,
}

impl<T, U> Coor<T, U> {
    fn mixup<V, W>(self, other: Coor<V, W>) -> Coor<T, W> {
        Coor {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list: Vec<i32> = vec![34, 35, 36, 100, 44, 66, 77, 88];

    let largest = get_largest(number_list);

    println!("Largest int is {}", largest);

    let u8_list: Vec<u8> = vec![100, 70, 50];

    let largest_char = get_largest(u8_list);

    println!("Largest u8 is {}", largest_char);

    let float_point = Point { x: 1.7, y: 1.4 };

    println!("{:#?}", float_point);

    let integer_point = Point { x: 16, y: 17 };

    println!("{:#?}", integer_point);

    let p = Point { x: 1.0, y: 1.4 };
    println!("{}", p.y_val());

    let p = Point { x: 1, y: 1 };
    println!("{}", p.x_val());

    let c1 = Coor { x: 1, y: 1.0 };
    let c2 = Coor { x: "Hello", y: "c" };
    println!("Coor = {:?}", c1.mixup(c2));
}

fn get_largest<T: PartialOrd + Copy>(num_list: Vec<T>) -> T {
    let mut largest = num_list[0];

    for number in num_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
