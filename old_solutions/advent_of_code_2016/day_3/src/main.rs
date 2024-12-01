use std::fs;

// Almost one liner
// Also really bad code but was fun

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut vektor: Vec<i32> = Vec::new();

    for line in input.trim().split("\n") {
        vektor.push(
            line.split_whitespace().collect::<Vec<&str>>()[0]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        );
    }

    for line in input.trim().split("\n") {
        vektor.push(
            line.split_whitespace().collect::<Vec<&str>>()[1]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        );
    }

    for line in input.trim().split("\n") {
        vektor.push(
            line.split_whitespace().collect::<Vec<&str>>()[2]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        );
    }

    let num: i32 = input
        .trim()
        .split("\n")
        .map(|x| {
            let sides: Vec<i32> = x
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("jeste ovde"))
                .collect();

            let is_valid = (sides[0] < sides[1] + sides[2])
                && (sides[1] < sides[0] + sides[2])
                && (sides[2] < sides[0] + sides[1]);

            if is_valid {
                return 1;
            }
            0
        })
        .fold(0, |acc, val| acc + val);

    println!("{}", num);

    let num: i32 = vektor.chunks(3).fold(0, |acc, val| {
        acc + ((val[0] < val[1] + val[2]
            && (val[1] < val[0] + val[2])
            && (val[2] < val[0] + val[1])) as i32)
    });

    println!("{}", num);
}
