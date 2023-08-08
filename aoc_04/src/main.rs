use std::cmp::Ordering;
use itertools::izip;

fn num_to_digit(num: i32) -> Vec<i32> {
    fn num_digit_inner(num: i32, digits: &mut Vec<i32>) {
        if num > 10 {
            num_digit_inner(num / 10, digits);
        }
        digits.push(num % 10);
    }
    let mut xs = Vec::new();
    num_digit_inner(num, &mut xs);
    xs
}

fn recursiv_count(length: usize, prev_max: i32, double_digit: bool, minimum: &Vec<i32>, mut greater: bool, maximum: &Vec<i32>, mut less: bool) -> i32 {
    if length == 6 {
        return if double_digit {1} else {0};
    }

    let min_d = minimum.get(length).unwrap().clone();
    let max_d = maximum.get(length).unwrap().clone();

    let mut sum = 0;
    for i in prev_max.max(1)..10 {
        let new_greater = if greater { true }
        else {
            match i.cmp(&min_d) {
                Ordering::Greater => true,
                Ordering::Less => continue,
                Ordering::Equal => false
            }
        };

        let new_less = if less { true }
        else {
            match i.cmp(&max_d) {
                Ordering::Less => true,
                Ordering::Greater => continue,
                Ordering::Equal => false
            }
        };

        if i == prev_max {
            sum += recursiv_count(length + 1, prev_max, true, minimum, new_greater, maximum, new_less);
        } else {
            sum += recursiv_count(length + 1, i, double_digit, minimum, new_greater, maximum, new_less);
        }

    }

    sum
}

fn recursiv_count2(length: usize, prev_max: i32, length_same: i32, double_digit: bool,
                   minimum: &Vec<i32>, mut greater: bool,
                   maximum: &Vec<i32>, mut less: bool) -> i32 {
    if length == 6 {
        return if double_digit || length_same == 2 {1} else {0};
    }

    let min_d = minimum.get(length).unwrap().clone();
    let max_d = maximum.get(length).unwrap().clone();

    let mut sum = 0;
    for i in prev_max.max(1)..10 {
        let new_greater = if greater { true }
        else {
            match i.cmp(&min_d) {
                Ordering::Greater => true,
                Ordering::Less => continue,
                Ordering::Equal => false
            }
        };

        let new_less = if less { true }
        else {
            match i.cmp(&max_d) {
                Ordering::Less => true,
                Ordering::Greater => continue,
                Ordering::Equal => false
            }
        };

        if i == prev_max {
            sum += recursiv_count2(length + 1, prev_max, length_same+1, double_digit,
                                   minimum, new_greater, maximum, new_less);
        } else {
            let double = length_same == 2 || double_digit;
            sum += recursiv_count2(length + 1, i, 1,
                                   double, minimum, new_greater, maximum, new_less);
        }

    }

    sum
}

fn solve1() {
    let count = recursiv_count(
        0, 0, false,
        &num_to_digit(153517), false,
        &num_to_digit(630395), false
    );

    println!("Answer 1: {}", count);
}

fn solve2() {
    let count = recursiv_count2(
        0, 0, 0, false,
        &num_to_digit(153517), false,
        &num_to_digit(630395), false
    );

    println!("Answer 2: {}", count);
}

fn main() {
    solve1();
    solve2();
}
