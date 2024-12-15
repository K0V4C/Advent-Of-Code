/*

    Solution for Advent of Code Day 8

*/

use std::collections::{HashMap, HashSet};
use std::fs;

fn generate_combination(vek: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    // Generate all pairs
    let mut all_pairs: Vec<((i32, i32), (i32, i32))> = vec![];

    let mut idx_first = 0;
    while idx_first < (vek.len() - 1) {
        let mut idx_second = idx_first + 1;
        while idx_second < vek.len() {
            all_pairs.push((vek[idx_first], vek[idx_second]));
            idx_second += 1;
        }
        idx_first += 1;
    }

    all_pairs
}
pub fn run() {
    println!("--- Started solving day x ---");

    let _test_file_path = "./src/day_8/test.txt";
    let _problem_file_path = "./src/day_8/problem.txt";

    // Attempt to read the file
    let file_content = match fs::read_to_string(_problem_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    // Initialize Athena map
    let mut anthena_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    // Calculate height and width
    let lines: Vec<&str> = file_content.lines().collect();
    let height = lines.len() as i32;
    let width = if height > 0 { lines[0].len() as i32 } else { 0 };

    let mut map_matrix: Vec<Vec<char>> = vec![];

    for line in file_content.lines() {
        let mut temp_vec = vec![];
        for character in line.chars() {
            temp_vec.push(character);
        }
        map_matrix.push(temp_vec);
    }

    for i in 0..width {
        for j in 0..height {
            match map_matrix[i as usize][j as usize] {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '.' => {
                    if map_matrix[i as usize][j as usize] != '.' {
                        anthena_map
                            .entry(map_matrix[i as usize][j as usize])
                            .and_modify(|vec| vec.push((i as i32, j as i32)))
                            .or_insert(vec![(i as i32, j as i32)]);
                    }
                }
                _ => {
                    panic!("Incorrect input")
                }
            };
        }
    }

    // Now just calculate all locations and check if they are valid

    let mut antinode_set: HashSet<(i32, i32)> = HashSet::new();

    for (_anthena, vector) in anthena_map.clone() {
        let all_pairs = generate_combination(&vector);
        for (first_pair, second_pair) in all_pairs {
            let x_distance = first_pair.0 - second_pair.0;
            let y_distance = first_pair.1 - second_pair.1;

            let mut possible_locations: Vec<(i32, i32)> = vec![];

            possible_locations.push((first_pair.0 + x_distance, first_pair.1 + y_distance));

            possible_locations.push((second_pair.0 - x_distance, second_pair.1 - y_distance));

            for location in possible_locations {
                if (location.0 >= 0 && location.0 < width)
                    && (location.1 >= 0 && location.1 < height)
                    && (location != first_pair)
                    && (location != second_pair)
                {
                    antinode_set.insert(location);
                }
            }
        }
    }

    println!(
        "Count of all possible antinode locations is >>> {}",
        antinode_set.len()
    );

    println!("--- Started solving day 8 part 2 ---");

    antinode_set.clear();

    for (_anthena, vector) in anthena_map {
        let all_pairs = generate_combination(&vector);

        for (first_pair, second_pair) in all_pairs {

            let x_distance = first_pair.0 - second_pair.0;
            let y_distance = first_pair.1 - second_pair.1;

            antinode_set.insert(first_pair);
            antinode_set.insert(second_pair);

            let mut location = (first_pair.0 + x_distance, first_pair.1 + y_distance);
            loop {
                if (location.0 >= 0 && location.0 < width)
                    && (location.1 >= 0 && location.1 < height)
                {
                    antinode_set.insert(location);
                } else {
                    break;
                }
                location.0 += x_distance;
                location.1 += y_distance;
            }

            location = (second_pair.0 - x_distance, second_pair.1 - y_distance);
            loop {
                if (location.0 >= 0 && location.0 < width)
                    && (location.1 >= 0 && location.1 < height)
                {
                    antinode_set.insert(location);
                } else {
                    break;
                }
                location.0 -= x_distance;
                location.1 -= y_distance;
            }
        }
    }

    println!("SOLUTION FOR PART 2 >> {}", antinode_set.len());

    println!("--- All done! ---");
}
