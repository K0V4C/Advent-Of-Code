/*

    Solution for Advent of Code Day 1

*/

use std::collections::HashMap;

pub fn run() {

    println!("--- Started solving day 1 ---");

    let _test_file_path = "./src/day_1/test.txt";
    let _problem_file_path = "./src/day_1/problem.txt";

    let read_lines = std::fs::read_to_string("./src/day_1/problem.txt").unwrap();

    let str_number = read_lines.split_ascii_whitespace().collect::<Vec<&str>>();

    let mut left_numbers:Vec<i32> = vec![];
    let mut right_numbers:Vec<i32> = vec![];

    let num_range = 0..(str_number.len() / 2);

    for idx in num_range.clone() {

        left_numbers.push(str_number[idx*2].parse().unwrap());
        right_numbers.push(str_number[idx*2 + 1].parse().unwrap());
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut distance: i32 = 0;

    for idx in num_range.clone() {
        distance += (left_numbers[idx] - right_numbers[idx]).abs();
    }

    println!("PART 1 SOLUTION >>>    {} ", distance);

    println!("--- Started solving day 1 part 2 ---");

    let mut right_index: HashMap<i32, i32>  = HashMap::new();

    for e in right_numbers {
        
        right_index.entry(e).and_modify(|x| *x += 1).or_insert(1);

    }

    let mut new_distance = 0;

    for e in left_numbers {

        let x_times = right_index.get(&e);

        if let Some(val) = x_times {
            new_distance += e * *val;
        }

    }

    println!("PART 1 SOLUTION >>>    {} ", new_distance);
    
    println!("--- All done! ---");

}