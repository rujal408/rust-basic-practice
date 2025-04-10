fn main() {
    assert_eq!(make_negative(1), -1);
    assert_eq!(make_negative(-5), -5);
    assert_eq!(make_negative(0), 0);

    let c: char = 'A';
    assert_eq!(get_grade(100, 100, 100), c);
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
