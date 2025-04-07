mod game;
fn main() {
    // Not Mutable
    let num: u8 = 255;
    println!("This is stored in num {}", num);

    // Mutable variable
    let mut numer: i8 = -33;
    println!("This is stored in num {}", numer);
    numer = -90;
    println!("This is stored in num {}", numer);

    // String lateral Variable
    let string_literal: &str = "Hi Rujal!!";
    println!("This is string literal {}", string_literal);

    // Convert &str to String
    let mut main_string: String = String::from("Hi Rujal!!");
    main_string.push_str("Pushed"); // Append to main_string
    println!("This is main string {}", main_string);

    // In &str we cannot append like String so String is dynamic
    // &str is fixed length string
    // String is allocated in heap memory

    let emp_info: (&str, u8) = ("Rujal", 60);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    println!("Emp name is {} and emp age is {}", emp_name, emp_age);

    // We can also do destructuring
    let (name, age) = emp_info;
    println!("Destructuring Emp name is {} and emp age is {}", name, age);

    print_value();
    print_number(2);

    let num1: u8 = 33;
    let num2: u8 = 44;
    let result: u8 = add(num1, num2);
    println!("{}", result);

    outside_variable_fn();
    ownership_transfer_example();
    referencing_dereferencing();
    dangling_reference();
    array_tutorial();
    vector_tutorial();
    shadowing_tutor();
    loop_tutor();
    match_tutor();
    input_output();
    game::guessing_game()
}

fn print_value() {
    println!("Hello print value function");
}

fn print_number(n: u8) {
    println!("The number is {}", n);
}

fn add(a: u8, b: u8) -> u8 {
    return a + b;
}

fn outside_variable_fn() {
    let outside_var = 5;

    {
        let inside_variable = 10;
        println!("Inside {}", inside_variable);
    }

    println!("The outsidebariable {}", outside_var);
}

fn ownership_transfer_example() {
    // CORRECT
    let a = 1;
    let b = a;
    println!("a={}", a);
    println!("b={}", b);

    // WRONG as same value cannot have more than one pointer
    //let str1 = String::from("Hello!"); //str1 is owner of Hello and we can have only one owner
    //let str2 = str1; //transfer of ownership
    // println!("str1={}", str1); //but we tried to access str1 whose ownership is already transfered so error
    //println!("str2={}", str2);

    let c: i8 = 6;
    println!("c={}", c);
    process_int(c);

    fn process_int(x: i8) {
        println!("x={}", x);
    }

    // let str3: String = String::from("Hello3!");
    // process_str(str3); //new new owner is item so
    // println!("str3={}", str3); // this will fail

    // fn process_str(item: String) { // here now owner is here
    //     println!("st3={}", item);
    // }

    let st = String::from("Hello");
    let (st2, len) = calculate_length(st);

    println!("Length of {} is {}", st2, len);

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        return (s, length); //return ownership transfer, 5
    }
    // if st.clone() is done then no ownership is transfered EG: calculate_length(st.clone());
    // st.clone() is deepcopy of data hence it utilizes more memory (EXPENSIVE MEMORY)

    // Another technique is transfer the reference of st (BORROW Operation)
    let s1 = String::from("Hello");
    let l = cal_len(&s1); // borrow operation
    println!("{} {}", s1, l);
    fn cal_len(s2: &String) -> usize {
        return s2.len(); //AUTO DE-REFERENCING
    }

    // To make borrow mutable provide mutable in reference
    let mut ss = String::from("Hello");
    let l = len_calc(&mut ss); // borrow operation

    println!("{} {}", ss, l);

    // let w2 = &mut ss;
    // w2.push_str("Wor");

    // let w3 = &mut ss;
    // println!("Another reference {} and {}", w2, w3); //TWO pointer cannot be read at once

    fn len_calc(s2: &mut String) -> usize {
        s2.push_str("WOrld"); //making changes or mutating
        return s2.len();
    }

    // NOTE: If one mutable reference is defined use it and finish it before another mutable reference is defined
}

fn referencing_dereferencing() {
    let mut x = 5;
    let y = &mut x;
    // println!("{:p}", &x);
    // y = y + 1; // NOTE: we cannot add
    // need to dereference
    *y = *y + 1;
    println!("y={}", y);
}

fn dangling_reference() {
    // let reference = create_ref();

    // fn create_ref() -> &String {
    //     let s: String = String::from("HH"); //
    //     return &s;
    // }
}

// Fixed size or length
fn array_tutorial() {
    let mut arr1: [u8; 5] = [1, 2, 3, 4, 5]; // Array declaration
    arr1[1] = 9;
    println!("=>{}", arr1[1]);

    let arr: [&str; 3] = ["Hello", "World", "Coders"];
    write_arr(arr); //Array passed directly
    println!("arr=>{:?}", arr);

    fn write_arr(mut a: [&str; 3]) {
        // This is expensive method
        a[0] = "Hell";
        println!("a=>{:?}", a);
    }
    let mut arr2: [&str; 3] = ["Hello", "World", "Coders"];

    write_arr2(&mut arr2);

    println!("arr2=>{:?}", arr2);

    // This is the good practice and it is not expensive
    fn write_arr2(a2: &mut [&str; 3]) {
        // passing a reference will motify the original array which is arr2 as s2 will point to arr2
        a2[0] = "Hell";
        println!("a2=>{:?}", a2);
    }
}

// Dynamic Array -> Vector (dynamic size)
fn vector_tutorial() {
    let mut v: Vec<i32> = Vec::new();
    // let mut v2 = Vec::<i32>::new(); // Another way of declaration

    v.push(1);
    v.push(2);
    v.push(3);

    println!("v=>{:?}", v);

    let mut v2 = vec![6, 7, 8, 9];
    v2.push(10);
    println!("v2=>{:?}", v2);
    v2.pop();
    println!("v2=>{:?}", v2);

    let vrr: Vec<&str> = vec!["Hello", "World", "Coders"]; // vrr is the owner of this vector for dynamic case
    write_vrr(&vrr);
    println!("{:?}", vrr); //This print will cause error if &vrr (Reference) is not passed in write_wrr instead of vrr
    fn write_vrr(vrr2: &Vec<&str>) {
        println!("{:?}", vrr2)
    }
}

// Can Redeclare the same variable with different data type
fn shadowing_tutor() {
    // let x = 10; //integer
    // let x = "Hello"; //string
    // let x = x.len(); //integer

    // println!("{}", x)
}

fn loop_tutor() {
    let arr = [1, 2, 3, 4, 5, 6];

    for element in &arr {
        println!("element {}", element)
    }
}

fn match_tutor() {
    let number = 5;

    match number {
        1 => println!("Number is one"),
        2 => println!("Number is two"),
        3 => println!("Number is three"),
        4 => println!("Number is four"),
        5 => println!("Number is five"),
        6 => println!("Number is six"),
        _ => println!("None matched"),
    }

    fn is_even(num: &i32) -> bool {
        return num % 2 == 0;
    }
    let n: i32 = 4;

    // Here i have passed &n we can pass n directly but i passed reference here
    match &n {
        x if is_even(x) => println!("Even"),
        x if !is_even(x) => println!("Odd"),
        _ => println!("Number not recognzed"),
    }
}

fn input_output() {
    // let mut input = String::new();
    // println!("Please input your name:");
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");

    // println!("Input name : {}", input);
}
