use std::collections::HashMap;

fn main() {
    let a = [1, 2, 3, 4];

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![4, 5, 6, 7];

    // let third = &v2[20];
    // // error occurs because length is only 3
    // println!("{}", third)

    // This will not throw error hence we can handle index case without error with this
    match v2.get(20) {
        Some(third) => println!("{}", third),
        _ => println!("No third element"),
    }

    let hello = String::from("Hello");
    // let c: char = hello[0]; // in low level language we cannot index string
    // And each character has 2 bytes

    for b in hello.bytes() {
        println!("{}", b)
    }

    for b in hello.chars() {
        println!("{}", b)
    }

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 66);

    scores.entry(String::from("Blue")).or_insert(90); // Update Hash Map
    count_words()
}

fn count_words() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
