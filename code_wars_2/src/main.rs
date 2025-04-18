fn main() {
    println!("Hello, world!");
    longest("xyaabbbccccdefww", "xxxxyyyyabklmopq");
}

// a = "xyaabbbccccdefww"
// b = "xxxxyyyyabklmopq"
// longest(a, b) -> "abcdefklmopqwxy"

// a = "abcdefghijklmnopqrstuvwxyz"
// longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
fn longest(a1: &str, a2: &str) {
    let a = a1.chars().collect::<Vec<char>>();
    let b = a2.chars().collect::<Vec<char>>();

    let mut c = a
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
