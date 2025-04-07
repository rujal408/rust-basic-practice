fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x; // The lifetime of x ends here hence error when printed below
    // }

    // println!("r: {}", r);

    let string1 = String::from("Yes");
    let string2 = String::from("sdfsdf");

    let result = longest(string1.as_str(), string2.as_str());

    println!("{}", result);

    // This will pass
    // let str1 = String::from("abc");
    // {
    //     let str2 = String::from("slkdfsf");
    //     let res = longest(str1.as_str(), str2.as_str());
    //     println!("{}", result)
    // }

    // This will fail as str2 (within that scope lifetime ends) have different lifetime than that of str1
    // res will last within that end of the scope
    // let str1 = String::from("abc");
    // let res;

    // {
    //     let str2 = String::from("slkdfsf");
    //     res = longest(str1.as_str(), str2.as_str());
    // }
    // println!("{}", res)
}

// here x and y can have different life times so return type will be error
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// &i32 // a reference
// &'a i32 // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
