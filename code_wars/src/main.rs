fn main() {
    assert_eq!(make_negative(1), -1);
    assert_eq!(make_negative(-5), -5);
    assert_eq!(make_negative(0), 0);

    let c: char = 'A';
    assert_eq!(get_grade(100, 100, 100), c);
    binary_slice_to_number([1, 0, 0, 0]);

    assert_eq!(binary_slice_to_number(&vec![0, 0, 0, 1]), 1);
    assert_eq!(binary_slice_to_number(&vec![0, 0, 1, 0]), 2);
    assert_eq!(binary_slice_to_number(&vec![1, 1, 1, 1]), 15);
    assert_eq!(binary_slice_to_number(&vec![0, 1, 1, 0]), 6);

    let a: &str = "This website is for losers LOL!";
    disemvowel(a)
}

fn make_negative(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n > 0 {
        -n
    } else {
        n
    }

    // BEST Techniques
    // -n.abs()
    //
    // match n.is_positive() {
    //     true => -n,
    //     false => n,
    // }
}

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    match (s1 + s2 + s3) / 3 {
        90..=100 => 'A',
        80..=90 => 'B',
        70..=80 => 'C',
        60..=70 => 'D',
        _ => 'F',
    }
}

// Convert time to milli seconds
fn past(h: i32, m: i32, s: i32) -> i32 {
    let hour_milli = h * 60 * 60 * 1000;
    let min_milli = m * 60 * 1000;
    let sec_milli = s * 1000;

    hour_milli + min_milli + sec_milli

    // BEST Practice
    // ((h * 60 + m) * 60 + s) * 1000
    // h*3600000 + m*60000 + s*1000
}

// TO SOLVE: [1,2,3]=>1*2*3=6
fn grow(nums: Vec<i32>) -> i32 {
    let mut result = 1;
    for num in nums {
        result *= num;
    }
    return result;
    // BEST Solution
    // nums.iter().product()
}

// TO SOLVE: [1,-2,3,6]=>1*3*6
// fn positive_sum(arr: &[i32]) -> i32 {
//     arr.iter().filter(|x| x.is_positive()).sum()
// }

fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut v = Vec::new();

    for i in 1..=n {
        v.push(x * i);
    }
    v

    // SOLID Solution
    // (1..=n).map(|e| x*e).collect()
}

fn remove_exclamation_marks(input: &str) -> String {
    input.chars().filter(|c| *c != '!').collect()

    // SOLID SOLUTION
    // input.replace("!", "")
    //
}

// Rujal Sapkota into R.S
fn abbrev_name(name: &str) -> String {
    name.to_string()
        .split(" ")
        .map(|x| x.chars().nth(0).unwrap().to_ascii_uppercase().to_string())
        .collect::<Vec<String>>()
        .join(".")
}

// NOTE:
// 'a' is char type and "a" is &str type

fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let remaining = cap - on;
    if remaining >= wait {
        0
    } else {
        wait - remaining
    }

    // BEST SOLUTION
    // (on + wait - cap).max(0)
}

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.iter()
        .map(|d| {
            if (d.0 >= 55 && d.1 > 7) || (d.1 >= 55 && d.0 > 7) {
                "Senior".to_string()
            } else {
                "Open".to_string()
            }
        })
        .collect()
}

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    // your code here
    let n = slice
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    u32::from_str_radix(&n, 2).unwrap()

    // BEST SOLUTIONS
    //  slice
    // .iter()
    // .rev()
    // .enumerate()
    // .map(|(i, b)| b << i)
    // .sum()
}

fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().map(|(x, y)| x - y).sum()
}

fn disemvowel(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    s.to_string()
        .split("")
        .map(|x| {
            x.chars()
                .filter(|y| !vowels.contains(&y.to_ascii_lowercase()))
                .collect()
        })
        .collect::<Vec<String>>()
        .join("")
    // BEST SOLUTION
    // s.replace(['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'], "")
}

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.is_empty() {
        numbers.to_vec()
    } else {
        let mut smallest: u32 = numbers[0];
        let mut _ind: usize = 0;

        for (i, n) in numbers.iter().enumerate() {
            if n < &smallest {
                smallest = *n;
                _ind = i;
            }
        }
        let mut new_vec = numbers.to_vec();
        new_vec.remove(_ind);
        new_vec
    }

    // BEST SOLUTION
    // let mut numbers = numbers.to_vec();
    // match numbers.iter().position_min() {
    //     None => numbers,
    //     Some(m) => {numbers.remove(m); numbers}
    // }
}
