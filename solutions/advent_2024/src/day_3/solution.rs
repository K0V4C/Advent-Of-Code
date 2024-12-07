/*

    Solution for Advent of Code Day 3

*/

use std::result;

use regex::Regex;

pub fn run() {
    println!("--- Started solving day 3 ---");

    let _test_file_path = "./src/day_3/test.txt";
    let _problem_file_path = "./src/day_3/problem.txt";

    let read_file = std::fs::read_to_string(_problem_file_path).unwrap();

    let search_regex = Regex::new(r"mul\(([1-9][0-9]*|0),([1-9][0-9]*|0)\)").unwrap();

    let mut search_results: Vec<(i32, i32)> = vec![];

    for (_, [first_num, second_num]) in search_regex.captures_iter(&read_file).map(|c| c.extract())
    {
        search_results.push((first_num.parse().unwrap(), second_num.parse().unwrap()));
    }

    let mut result_value = 0;
    for (first, second) in search_results {
        result_value += first * second;
    }

    println!("SOLUTION FOR PART 1 >>>> {}", result_value);

    println!("--- Started solving day 3 part 2 ---");

    let mut is_enabled_mul = true;

    let search_regex: Regex =
        Regex::new(r"(mul\(([1-9][0-9]*|0),([1-9][0-9]*|0)\))|(do\(\))|(don't\(\))").unwrap();

    let mut final_result: i32 = 0;

    for capture in search_regex.captures_iter(&read_file) {
        let something = capture.iter().next().unwrap().map(|m| m.as_str());

        if let None = something {
            continue;
        }

        let value = something.unwrap();

        if value == "don't()" {
            is_enabled_mul = false;
        } else if value == "do()" {
            is_enabled_mul = true;
        } else {
            if is_enabled_mul == false {
                continue;
            }

            let number_regex = Regex::new(r"([1-9][0-9]*|0),([1-9][0-9]*|0)").unwrap();

            for (_, [left, right]) in number_regex.captures_iter(value).map(|m| m.extract()) {
                final_result += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
            }
        }
    }

    println!("SOLUTION FOR PART 2 >>>>>  {}", final_result);

    println!("--- All done! ---");
}
