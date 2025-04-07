#![allow(unused)]
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};

//Some commonly used macros
//panic!()
//assert!()
//assert_eq!()
//assert_ne!()

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn main() {
    let num = {
        let sec = 1;
        sec
    };

    println!("{}", num);

    let charlie = Animal {
        name: String::from("Charlie"),
        age: 1,
    };

    // print_item(charlie);
    // print_item(12);

    // compare_and_display("Listen up!", 2,4, charlie);

    let new_vec: Vec<i32> = vec![1, 2];
    let new_vec_2: Vec<i32> = vec![1, 2, 6, 7, 9, 3, 4];
    println!(
        "{:?}, {:?}",
        take_fifth(new_vec),
        take_fifth(new_vec_2).unwrap()
    );
    // or use .unwrap_or(0) this defaults 0 if none

    // Best way to handle this case optional or some
    let new_vec_3: Vec<i32> = vec![1, 2, 7, 6, 5, 4, 3];
    match take_fifth(new_vec_3) {
        Some(number) => println!("Number is {}", number),
        _ => println!("Too short vec"),
    }

    let mut result_vec = Vec::new();

    for number in 2..7 {
        result_vec.push(check_if_five(number))
    }

    println!("{:?}", result_vec);
    check_turbo_fish();
    collection_types();
}

fn take_fifth(val: Vec<i32>) -> Option<i32> {
    if val.len() < 5 { None } else { Some(val[4]) }
}

fn print_item<MyGenericType: Debug>(item: MyGenericType) {
    println!("Here is the item: {:?}", item)
}

// fn compare_and_display<T,U,V>(statement:T, num_1:U, num_2:U, an:V)
// where T:Display,
//       U:Display+PartialOrd,
//       V:Debug
// {
//     println!("{}! is {} greater than {}? {}",statement, num_1, num_2, num_1>num_2);
//     println!("{:?}", an);
// }

// pub enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }

use std::num::ParseIntError;

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five".to_string()),
    }
}

fn check_turbo_fish() {
    fn return_number(input: &str) -> Result<i32, ParseIntError> {
        input.parse::<i32>()
    };

    let v = vec!["5", "five", "9.7", "789"];
    let index = v.get(8);
    let index_2 = v.get(2);
    println!("With get {:?}, {:?} and {:?}", index, index_2, v);
    for number in v {
        match return_number(number) {
            Ok(number) => println!("Got a {}", number),
            Err(e) => println!("Didn't work: {}", e),
        }
    }

    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7"],
        vec!["Athens", "sunny", "20", "10"],
    ];

    println!("Weather {:?}", weather_vec);
}

use std::collections::{BTreeMap, HashMap, HashSet};

struct City {
    name: String,
    population: HashMap<u32, u32>,
}

struct BCity {
    name: String,
    population: BTreeMap<u32, u32>,
}

fn collection_types() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(1372, 3250);
    tallinn.population.insert(1851, 24000);

    for (year, population) in tallinn.population {
        println!(
            "In the year {}, the city of {} had a population of {}",
            year, tallinn.name, population
        );
    }

    let mut Ttallinn = BCity {
        name: "TTallinn".to_string(),
        population: BTreeMap::new(),
    };

    Ttallinn.population.insert(234, 2343);
    Ttallinn.population.insert(123, 34534);

    for (year, population) in Ttallinn.population {
        println!(
            "In the year {}, the city of {} had a population of {}",
            year, Ttallinn.name, population
        );
    }

    // How to count and save

    let book_collection = vec![
        "L' Allemagne Moderne",
        "Top of the world",
        "Time Management",
        "Time Management",
    ];

    let mut book_hashmap = HashMap::new();
    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        *return_value += 1;
    }

    for (book, number) in book_hashmap {
        println!("{},{}", book, number);
    }

    let mut many_numbers = vec![
        11, 22, 33, 44, 55, 66, 77, 88, 12, 34, 56, 78, 43, 65, 87, 54, 23, 54, 67, 23, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 22,
    ];
    let man_len = many_numbers.len();
    let mut number_hashset = HashSet::new();

    for num in many_numbers {
        number_hashset.insert(num);
    }
    println!(
        "Hashset length is {} out of {}",
        number_hashset.len(),
        man_len
    )
}
