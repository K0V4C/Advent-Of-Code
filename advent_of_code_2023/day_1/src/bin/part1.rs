use std::fs;
fn main() {
    let input = fs::read_to_string("./src/bin/input.txt");

    let data = match input {
        Ok(input) => input,
        Err(error) => panic!("Error reading file"),
    };

    let lines = data.split('\n').collect::<Vec<&str>>();

    let mut sum = 0;

    for line in lines.iter() {
        let mut first = 0;
        let mut last = 0;

        for x in line.chars() {
            if x.is_digit(10) {
                first = x.to_digit(10).unwrap();
                break;
            }
        }

        for x in line.chars().rev() {
            if x.is_digit(10) {
                last = x.to_digit(10).unwrap();
                break;
            }
        }

        println!("{first} {last}");

        sum += first * 10 + last;
    }

    println!("Total sum is: {}", sum);
}
