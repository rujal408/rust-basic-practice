use std::fmt::{Debug, Display, format};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("This is default")
    }

    fn summarize_author(&self) -> String;
}

// fn notify<T=Summary>(item: &impl T) {
//     println!("Breaking news! {}", item.summarize());
// }

// fn some_function<T, U>(t: &T, u: &U) -> i32 {
//     // ...
//     3
// }

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     //
// }

// fn some_function<T, U>(t: &T, u: &U) -> i32 // can define generics
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     4
// }

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Horse ebooks"),
        content: String::from("Of course, as you probably"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x={}", self.x)
        } else {
            println!("The largest member is y={}", self.y)
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@jondoe"),
        content: String::from("Hello world content"),
        reply: true,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    println!("Article author: {}", article.summarize_author());
    println!("Tweet author: {}", tweet.summarize_author());

    println!("{:?}", returns_summarizable().summarize());

    struct Animal {
        name: String,
    }

    trait Canine {
        fn bark(&self) -> String {
            String::from("Woof") // This is default value if not implmeneted in the struct
        }

        fn run(&self) -> String;
    }

    impl Canine for Animal {
        fn run(&self) -> String {
            String::from("Running")
        }

        fn bark(&self) -> String {
            format!("{} is running", self.name)
        }
    }

    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
    generics_traits();
}

fn generics_traits() {
    struct Elf {
        name: String,
    }
    struct Human {
        name: String,
    }

    trait Creature {
        fn get_name(&self) -> String {
            "Default".to_string()
        }
    }

    impl Creature for Elf {
        fn get_name(&self) -> String {
            format!("{}", self.name)
        }
    }

    impl Creature for Human {
        fn get_name(&self) -> String {
            format!("{}", self.name)
        }
    }

    fn which_creator<T>(creature: T)
    where
        T: Creature,
    {
        println!("Creature name: {}", creature.get_name());
    }

    let elf = Elf {
        name: String::from("Elf"),
    };
    let human = Human {
        name: String::from("Human"),
    };

    // Here below we take arguments from different types
    which_creator(elf);
    which_creator(human);
}
