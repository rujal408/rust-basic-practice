use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// #[derive(Debug)]
struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // let a = Cons(1, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(5, Box::new(a));

    // let a = Rc::new(Cons(1, Rc::new(Cons(10, Rc::new(Nil)))));

    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(5, Rc::clone(&a));
    // let b = Box::new(5);
    // println!("b = {b}");

    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // // type `MyBox<{integer}>` cannot be dereferenced
    // assert_eq!(5, *y);
    reference_count()
}

fn interior_mutability() {
    // Problem is here
    // let a = 5;
    // let b = &mut a;

    // let mut c = 10;
    // let d = &c;
    // *d = 20;
}

fn reference_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c: List = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
