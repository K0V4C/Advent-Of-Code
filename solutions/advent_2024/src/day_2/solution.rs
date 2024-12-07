/*

    Solution for Advent of Code Day 2

*/

use std::io::BufRead;

pub fn check_if_safe(numbers: Vec<i32>, dampener: bool) -> bool {
    if numbers.len() == 1 {
        return true;
    }

    let ascending = if numbers[0] < numbers[1] { true } else { false };

    let mut start_window_idx = 0;
    let mut end_window_idx = start_window_idx + 1;

    while end_window_idx < numbers.len() {
        let window = &numbers[start_window_idx..=end_window_idx];

        if (ascending == true && window[0] > window[1])
            || (ascending == false && window[0] < window[1])
            || (((window[0] - window[1]).abs() > 3) || ((window[0] - window[1]).abs()) < 1)
        {
            if dampener == false {
                return false;
            }

            for i in 0..numbers.len() {

                let mut new_numbers = numbers.clone();
                new_numbers.remove(i);

                if check_if_safe(new_numbers, false) {
                    return true;
                }
            }
            return false;
        }

        start_window_idx += 1;
        end_window_idx += 1;
    }

    true
}

pub fn run() {
    println!("--- Started solving day 2 ---");

    let _test_file_path = "./src/day_2/test.txt";
    let _problem_file_path = "./src/day_2/problem.txt";

    let read_lines: Vec<String> = std::fs::read(_problem_file_path)
        .unwrap()
        .lines()
        .map(|x| String::from(x.unwrap()))
        .collect();

    let mut safe_lines = 0;
    for line in read_lines {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if check_if_safe(numbers, false) == true {
            safe_lines += 1;
        }
    }

    println!("SOLUTION FOR PART 1 >>>> {safe_lines}");

    println!("--- Started solving day 2 part 2 ---");

    let read_lines: Vec<String> = std::fs::read(_problem_file_path)
        .unwrap()
        .lines()
        .map(|x| String::from(x.unwrap()))
        .collect();

    let mut safe_lines = 0;
    for line in read_lines {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if check_if_safe(numbers, true) == true {
            safe_lines += 1;
        }
    }

    println!("SOLUTION FOR PART 2 >>>> {safe_lines}");

    println!("--- All done! ---");
}
