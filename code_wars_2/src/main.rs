fn main() {
    longest("xyaabbbccccdefww", "xxxxyyyyabklmopq");
    rps("paper", "rock");
    println!("{}", high_and_low("1 2 3 4 5"))
}

// a = "xyaabbbccccdefww"
// b = "xxxxyyyyabklmopq"
// longest(a, b) -> "abcdefklmopqwxy"

// a = "abcdefghijklmnopqrstuvwxyz"
// longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
fn longest(a1: &str, a2: &str) -> String {
    let a = a1.chars().collect::<Vec<char>>();
    let b = a2.chars().collect::<Vec<char>>();

    let c = a
        .iter()
        .chain(b.iter())
        .map(|&x| x.to_string())
        .collect::<Vec<String>>();

    let mut d = remove_duplicates(c);
    d.sort();

    d.join("")

    // BEST SOLUTION
    // use std::collections::BTreeSet;
    // fn longest(a1: &str, a2: &str) -> String {
    //     a1.chars()
    //         .chain(a2.chars())
    //         .collect::<BTreeSet<char>>()
    //         .iter()
    //         .collect()
    // }
}

use std::collections::HashSet;
fn remove_duplicates(data: Vec<String>) -> Vec<String> {
    let set: HashSet<_> = data.into_iter().collect(); // Collect into a HashSet to remove duplicates
    set.into_iter().collect() // Convert back into a Vec
}

fn accum(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, x)| {
            let result = x.to_string().repeat(i + 1).to_lowercase();
            some_kind_of_uppercase_first_letter(&result)
        })
        .collect::<Vec<String>>()
        .join("-")
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn rps(p1: &str, p2: &str) -> &'static str {
    let items = vec!["rock", "paper", "scissors", "rock"];
    if p1 == p2 {
        "Draw!"
    } else {
        let index = items.iter().position(|&x| x == p1).unwrap_or(0);
        if items[index + 1] == p2 {
            "Player 2 won!"
        } else {
            "Player 1 won!"
        }
    }
}

pub fn remove_char(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, x)| {
            if i > 0 && i < s.len() - 1 {
                x.to_string()
            } else {
                "".to_string()
            }
        })
        .collect()

    // BEST SOLUTION
    //     s[1..s.len() - 1].to_string()
    //     s.chars().skip(1).take(s.chars().count() - 2).collect()
}

fn high_and_low(numbers: &str) -> String {
    let a = numbers
        .to_string()
        .split_whitespace()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max = a
        .iter()
        .fold(a[0], |acc, x| if *x > acc { *x } else { acc });

    let min = a
        .iter()
        .fold(a[0], |acc, x| if *x < acc { *x } else { acc });

    format!("{} {}", max, min)
}
