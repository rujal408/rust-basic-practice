use std::collections::HashMap;

fn main() {
    let mut vector_1 = vec![1, 2, 3];
    let vector_a = vector_1.iter().map(|value| value * 2).collect::<Vec<i32>>();
    // let vector_b = vector_1
    //     .into_iter()
    //     .map(|value| value * 10)
    //     .collect::<Vec<i32>>();

    println!("{:?}", vector_1);

    println!("{:?}", vector_a);
    // println!("{:?}", vector_b);
    //
    let mut vector_2 = vec![10, 20, 30];
    vector_2.iter_mut().for_each(|x| *x += 10);
    println!("{:?}", vector_2);

    // let my_vec = std::iter::repeat(6).skip(6).take(6);

    // println!("{:?}", my_vec)
    // Custom Iterator is made by own
    let mut my_library = Library::new();
    my_library.add_book("Book One");
    my_library.add_book("Book Two");
    my_library.add_book("Book Three");

    for book in my_library {
        println!("{}", book);
    }
    enumerate_work();
    zip_collect();
}

#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}
#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.books.pop() {
            Some(book) => Some(book + " is found!"),
            _ => None,
        }
    }
}

fn enumerate_work() {
    let num_vec = vec![10, 9, 8];

    let a = num_vec
        .into_iter()
        .enumerate()
        .map(|(index, mut number)| (index, number + 10))
        .collect::<Vec<_>>();

    println!("{:?}", a)
}

fn zip_collect() {
    let some_numbers = vec![0, 1, 2, 3];
    let some_words = vec!["zero", "one", "two", "three"];

    let number_hash_map = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect::<HashMap<_, _>>();

    println!("Converted to hashmap = {:?}", number_hash_map);
}
fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut v = Vec::new();

    for i in 1..=n {
        v.push(x * i);
    }
    v

    // SOLID Solution
    // (1..=n).map(|e| x*e).collect()
}
