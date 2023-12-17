use std::{fs, ops::Add};
fn main() {
    let input = fs::read_to_string("./src/bin/input.txt");

    let data = match input {
        Ok(input) => input,
        Err(error) => panic!("Error reading file"),
    }:

    let lines = data.split('\n').collect::<Vec<&str>>();

    let mut sum = 0;

    for line in lines.iter() {
        let mut first = 0;
        let mut last = 0;

        let mut index = 0;
        let mut new_line = String::from("");
        while index < line.len() {

            let reduced_line = &line[index..];

            if reduced_line.starts_with("one") {
                // index += "one".len();
                new_line = new_line + "1";
            } else if reduced_line.starts_with("two") {
                // index += "two".len();
                new_line = new_line + "2";
            } else if reduced_line.starts_with("three") {
                // index += "three".len();
                new_line = new_line + "3";
            } else if reduced_line.starts_with("four") {
                // index += "four".len();
                new_line = new_line + "4";
            } else if reduced_line.starts_with("five") {
                // index += "five".len();
                new_line = new_line + "5";
            } else if reduced_line.starts_with("six") {
                // index += "six".len();
                new_line = new_line + "6";
            } else if reduced_line.starts_with("seven") {
                // index += "seven".len();
                new_line = new_line + "7";
            } else if reduced_line.starts_with("eight") {
                // index += "eight".len();
                new_line = new_line + "8";
            } else if reduced_line.starts_with("nine") {
                // index += "nine".len();
                new_line = new_line + "9";
            } else {
                new_line.push(reduced_line.chars().next().unwrap());
            } 

            index += 1;


        }

        for x in new_line.chars() {
            if x.is_digit(10) {
                first = x.to_digit(10).unwrap();
                break;
            }
        }

        for x in new_line.chars().rev() {
            if x.is_digit(10) {
                last = x.to_digit(10).unwrap();
                break;
            }
        }

        sum += first * 10 + last;
    }

    println!("Total sum is: {}", sum);
}
