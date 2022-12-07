use std::str;
use std::fs;

fn get_flag1(word:&Vec<char>) -> bool{

    for x in 1..word.len()-1{
        let pair = &[word[x-1], word[x]];

        for y in x+1..word.len()-1{
            
            if pair == &[word[y], word[y+1]]{
                return true;
            }
        }
    }
    return false;
}

fn get_flag2(word:&Vec<char>) -> bool{
    for x in 2..word.len(){
        let first_char = word[x-2];
        let third_char = word[x];

        if first_char == third_char {
            return true;
        }
    }
    return false;

fn main(){
    let data = fs::read_to_string("data.txt").unwrap();
    let split = data.split("\n").collect::<Vec<&str>>();

    let mut nice_strings:i32 = 0;

    for x in split{
        let word = x.to_owned().clone();

        let word = word.chars().collect::<Vec<char>>();

        let flag1 = get_flag1(&word);

        let flag2 = get_flag2(&word);

        if flag1 && flag2 {
            nice_strings = nice_strings + 1;
        }
    
        // for letter in word.chars(){
        //     match letter{
        //         'a' => num_of_vocals = num_of_vocals+1,
        //         'e' => num_of_vocals = num_of_vocals+1,
        //         'i' => num_of_vocals = num_of_vocals+1,
        //         'o' => num_of_vocals = num_of_vocals+1,
        //         'u' => num_of_vocals = num_of_vocals+1,
        //         _ => continue,
        //     }
        // }
        // let mut flag1:bool = false;
        // let mut flag2 = true;
        // let word = word.as_bytes();
        
        // for x in 1..word.len(){
        //     let curr = word[x];
        //     let bef = word[x-1];
        //     if curr == bef {
        //         flag1 = true;                
        //     }
            
        //     match &[bef, curr] {
        //         b"ab"| b"cd" | b"pq" | b"xy" => {flag2 = false; break}
        //         _ => {},
        //     }
        // }

        // if num_of_vocals >= 3 && flag1 && flag2 {
        //     nice_strings = nice_strings + 1;
        //     print!("BANG");
        // }
    }
    println!("{}", nice_strings);
    
}
