use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt"); 

    let data = match data {
        Ok(data) => data,
        Err(error) => panic!("Error reading file: {}", error),
    };

    let elfes = data.split('\n').collect::<Vec<&str>>();

    let mut max = 0;
    let mut current_elf = 0;
    for x in elfes {
        if x == "" {
            if current_elf > max {
                max = current_elf;
            }
            current_elf = 0;
            continue;
        }

        let val: Result<i32, _> = x.parse(); 
        let val = match val {
            Ok(val) => val,
            Err(error) => panic!("we are gonna die {}", error),
        };

        current_elf += val;
    }

    println!("{}", max);

    let data = fs::read_to_string("data.txt").unwrap();
    let elfes: Vec<&str> = data.split("\n").collect();

    let mut top_3 = vec![0, 0, 0];
    let mut current_elf = 0;
    for x in elfes {
        if x == "" {
            if current_elf > top_3[2] {
                top_3[0] = top_3[1];
                top_3[1] = top_3[2];
                top_3[2] = current_elf;
            } else if current_elf > top_3[1] {
                top_3[0] = top_3[1];
                top_3[1] = current_elf;
            } else if current_elf > top_3[0] {
                top_3[0] = current_elf;
            }

            current_elf = 0;
            continue;
        }

        let val = x.parse::<i32>().unwrap();
        current_elf += val;
    }

    let sum_of_3 = top_3.iter().sum::<i32>();
    println!("Sum top  3: {}", sum_of_3);
}
