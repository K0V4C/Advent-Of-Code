
fn main(){

    let data = std::fs::read_to_string("data.txt").unwrap();
    let data = data.split_whitespace().collect::<Vec<&str>>();

    let mut score = 0;

    for x in data {

        let line = x.split(",").collect::<Vec<&str>>();

        let first_pair = line[0].split("-").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let second_pair = line[1].split("-").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        if first_pair[0] <= second_pair[0] && first_pair[1] >= second_pair[1] {
            score += 1;
        } else if second_pair[0] <= first_pair[0] && second_pair[1] >= first_pair[1] {
            score+= 1;
        }
    } 

    println!("{}", score);


    let data = std::fs::read_to_string("data.txt").unwrap();
    let data = data.split_whitespace().collect::<Vec<&str>>();

    let mut score = 0;

    
    for x in data {

        let line = x.split(",").collect::<Vec<&str>>();

        let first_pair = line[0].split("-").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let second_pair = line[1].split("-").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        let first_range = first_pair[0]..first_pair[1]+1;
        let first_range = first_range.collect::<Vec<i32>>();

        let second_range = second_pair[0]..second_pair[1]+1;
        let second_range = second_range.collect::<Vec<i32>>();

        
        for val in first_range.iter(){
            if second_range.contains(&val) { 
                score += 1;
                break;
            }
        }

    }

    println!("{}", score)

}