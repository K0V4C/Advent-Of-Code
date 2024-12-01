use std::fs;
//This isn't really a good code just was trying out different aspects of Rust 

fn _run() {
    let contents = match fs::read_to_string("input.txt"){
        Err(_) => panic!("File not found"),
        Ok(val) => val,
    };
    
    let mut dir = 0;

    let value =  contents.split(", ").map(|x| {

        let val: i32;

        match x.as_bytes()[0]{
            b'R' => {
                dir = (dir + 1) % 4;
                let (_, sufix) = x.split_at(1);
                val = sufix.parse::<i32>().expect("boom");
            }, 

            b'L' => {
                dir = (dir + 3) % 4;
                let (_, sufix) = x.split_at(1);
                val = sufix.parse::<i32>().expect("boom");
            }, 

            _ => panic!("non exist"),
        }; 

        return (dir, val);
    }).fold((0,0), |mut acc: (i32, i32), (dir, val)|{
        
        match dir {
            0 => {
                acc.0 += val;
            }, 
            1 => {
                acc.1 += val;
            },
            2 => {
                acc.0 -= val;
            },
            3 => {
                acc.1-= val;
            },
            _ => panic!("Wont happen"),
        };

        (acc.0, acc.1)
    });

    println!("{:?}", value.0 + value.1);
}


//This code here is better, but I guess the problem writing functional was having code under in mind while writing functional

use std::collections::HashSet;

fn move_traveler(position: &mut Position, instruction: &str) -> Option<(i32, i32)> {
    let direction = &instruction[..1];
    let distance = instruction[1..].parse::<i32>().unwrap();

    match direction {
        "R" => position.direction = (position.direction + 1) % 4,
        "L" => position.direction = (position.direction + 3) % 4,
        _ => (),
    }

    for _ in 0..distance {
        match position.direction {
            0 => position.y += 1,
            1 => position.x += 1,
            2 => position.y -= 1,
            3 => position.x -= 1,
            _ => (),
        }

        let location = (position.x, position.y);

        if position.visited.contains(&location) {
            return Some(location);
        }

        position.visited.insert(location);
    }

    None
}

struct Position {
    x: i32,
    y: i32,
    direction: i32,
    visited: HashSet<(i32, i32)>,
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let instructions = input.trim().split(", ");

    let mut position = Position {
        x: 0,
        y: 0,
        direction: 0,
        visited: HashSet::new(),
    };

    for instruction in instructions {
        if let Some(location) = move_traveler(&mut position, instruction) {
            println!(
                "{}",
                location.0.abs() + location.1.abs()
            );
            break;
        }
    }
}
