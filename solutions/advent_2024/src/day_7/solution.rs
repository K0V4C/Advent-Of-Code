/*

    Solution for Advent of Code Day 7

*/

fn can_reach_concat(start: usize, target: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return start == target;
    }

    if start > target {
        return false;
    }

    let (first, rest) = numbers.split_first().unwrap();
    can_reach_concat(start * first, target, rest)
        || can_reach_concat(start + first, target, rest)
        || can_reach_concat(concat(start, *first), target, rest)
}

fn concat(a: usize, b: usize) -> usize {
    let mut offset = 1;

    while offset <= b {
        offset *= 10;
    }

    
    a * offset + b
}
pub fn run() {


    println!("--- Started solving day 7 ---");

    let _test_file_path = "./src/day_7/test.txt";
    let _problem_file_path = "./src/day_7/problem.txt";

    let read_file = std::fs::read_to_string(_problem_file_path).unwrap();

    let mut calculations: Vec<(i64, Vec<i64>)> = vec![];

    for line in read_file.lines() {
        let split_it: Vec<&str> = line.trim().split(':').collect();
        let target =     split_it[0].parse::<i64>().unwrap();
        let numbers = split_it[1].split_whitespace().map(|m| m.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        calculations.push(
            (target,
            numbers)
        );
    }

    let mut total = 0;

    for (target, numbers) in calculations.clone() { 

        let range = (numbers.len() - 1) as u32;
        let base:i32 = 2;


        for i in 0..base.pow(range) {
            let mut idx = 0;
            let mut numbers:Vec<i64> = numbers.iter().cloned().rev().collect();
            while numbers.len() > 1{

                let first = numbers.pop().unwrap();
                let second = numbers.pop().unwrap();

                match (i & (1 << idx)) >> idx {
                    0 => {
                        numbers.push(first + second);
                    },
                    1 => {
                        numbers.push(first * second);
                    },
                    _ => {panic!("this should not happen")}
                };

                idx += 1;
            }

        
            let result = numbers.pop().unwrap();

            if result == target {
                total += target;
                break;
            }
        }
    }

    println!("This is the result for PART 1 >>>> {}", total);

    println!("--- Started solving day 7 part 2 ---");
    let input = std::fs::read_to_string(_problem_file_path).unwrap();

    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();

    for line in input.lines() {
        let (result, numbers) = line.split_once(':').unwrap();
        let result = result.parse().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        equations.push((result, numbers));
    }

    let mut result = 0;
    for (target, numbers) in equations {
        let (start, numbers) = numbers.split_first().unwrap();
        if can_reach_concat(*start, target, numbers) {
            result += target;
        }
    }

    println!("This is the result for PART 2 >>>> {}", result);

    println!("--- All done! ---");

}