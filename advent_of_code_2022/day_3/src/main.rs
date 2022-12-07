use std::{io::Error, collections::HashSet};

const BIG_MAGIC_CONST:u32 = 64;
const SMALL_MAGIC_CONST:u32 = 96;

fn calculate_score(first_elf: &HashSet<char>,  second_elf: &HashSet<char>, third_elf: &HashSet<char>) -> u32 {

    let first = first_elf.intersection(&second_elf).copied().collect::<HashSet<char>>();
    let second = first_elf.intersection(&third_elf).copied().collect::<HashSet<char>>();
    let final_char = first.intersection(&second).copied().collect::<HashSet<char>>();

    let found_char = final_char.iter().next().unwrap().clone();

    if found_char.is_uppercase() {
        return (found_char as u32) - BIG_MAGIC_CONST + 26;
    } else {
        return  (found_char as u32) - SMALL_MAGIC_CONST;
    }
}

fn main(){

    let data:Result<String, Error> = std::fs::read_to_string("data.txt");
    let data = match data {
        Ok(data) => data,
        Err(error) => panic!("Error reading file: {}", error)
    };

    let data_in_lines = data.split_whitespace();

    let mut score = 0;
 
    for x in data_in_lines {

        let mut first_comparment:Vec<char> = vec![];
        let len = x.len();
        let into_chars:Vec<char> = x.chars().collect();
        let mut i = 0;
        while i < len/2 {

            if !first_comparment.contains(&into_chars[i]) {
                first_comparment.push(into_chars[i]); 
            }
            i += 1;
        }

        while i < len {

            if first_comparment.contains(&into_chars[i]){
                if into_chars[i].is_uppercase() {
                    score += (into_chars[i] as u32) - BIG_MAGIC_CONST + 26;
                } else {
                    score += (into_chars[i] as u32) - SMALL_MAGIC_CONST;
                }

                let index = first_comparment.iter().position(|&r| r == into_chars[i]).unwrap();
                first_comparment.remove(index);
            }

            i+=1;
        }   
    }

    println!("Sum all: {:?}", score);


    let data =  std::fs::read_to_string("data.txt").expect("BOOM");
    let data = data.split_whitespace().collect::<Vec<&str>>();

    let mut first_elf:HashSet<char> = HashSet::new();
    let mut second_elf:HashSet<char> = HashSet::new();
    let mut third_elf:HashSet<char> = HashSet::new();
    
    let mut score = 0;
    
    let mut cnt = 0;

    for x in data{

        if cnt == 0 {
            first_elf = x.chars().collect();
        } else if cnt == 1 {
            second_elf = x.chars().collect();
        } else {

            third_elf = x.chars().collect();

            score += calculate_score(&first_elf, &second_elf, &third_elf);

            cnt = 0;
            continue;
        }
       cnt += 1;
    }


    println!("Score za badges: {}", score);

}
