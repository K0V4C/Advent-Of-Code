fn main(){

    let data = std::fs::read_to_string("data.txt").unwrap();
    let data = data.split("\n").collect::<Vec<&str>>();

    let mut stacks:Vec<&str> = vec![];
    for x in &data{
        stacks.push(x.clone());
        if *x == "" {
            break;
        }
    }
    let len = stacks[stacks.len()-2].split_whitespace().collect::<Vec<&str>>().len();

    let mut stack_vec:Vec<Vec<char>> = vec![vec![]; len];


    for x in &data {
        for (i, char) in x.chars().enumerate() {
            if char.is_alphabetic(){
                stack_vec[i / 4].insert(0, char);
            }
        }
        if *x == ""{
            break;
        } 
    }

    let mut flag = true;
    for x in &data {
        if *x == ""{
            flag = false;
            continue;
        }
        if flag {
            continue;
        }
        let mut triplets = x.split_whitespace().collect::<Vec<&str>>();
        triplets.remove(0);
        triplets.remove(1);
        triplets.remove(2);

        let triplets = triplets.iter().map(|r| r.parse().unwrap()).collect::<Vec<u32>>();

        for _i in 0..triplets[0]{
            let first_index = (triplets[1]- 1) as usize;
            if stack_vec[first_index].len() == 0 {
                break;
            }
            let selected_char = stack_vec[first_index].pop().unwrap();

            let second_index = (triplets[2]- 1) as usize;
            stack_vec[second_index].push(selected_char);
        }
        
    }
    let mut result:Vec<char> = vec![];
    for x in stack_vec {
        let temp = x[x.len()-1];
        result.push(temp);
    }

    for elem in result {
        print!("{}", elem);
    }
    println!("");
    //********************
    //PART 2 OF THE PUZZLE
    //********************
    let data = std::fs::read_to_string("data.txt").unwrap();
    let data = data.split("\n").collect::<Vec<&str>>();

    let mut stacks:Vec<&str> = vec![];
    for x in &data{
        stacks.push(x.clone());
        if *x == "" {
            break;
        }
    }
    let len = stacks[stacks.len()-2].split_whitespace().collect::<Vec<&str>>().len();

    let mut stack_vec:Vec<Vec<char>> = vec![vec![]; len];

    for x in &data {
        for (i, char) in x.chars().enumerate() {
            if char.is_alphabetic(){
                stack_vec[i / 4].insert(0, char);
            }
        }
        if *x == ""{
            break;
        }
    }

    let mut flag = true;
    for x in &data {
        if *x == ""{
            flag = false;
            continue;
        }
        if flag {
            continue;
        }

        let mut triplets = x.split_whitespace().collect::<Vec<&str>>();
        triplets.remove(0);
        triplets.remove(1);
        triplets.remove(2);

        let triplets = triplets.iter().map(|r| r.parse().unwrap()).collect::<Vec<u32>>();

        let first_index = (triplets[1]- 1) as usize;
        let second_index = (triplets[2]- 1) as usize;
        let mut size = triplets[0] as usize;

        let insert_index = stack_vec[second_index].len();

        if size > stack_vec[first_index].len(){
            size = stack_vec[first_index].len();
        }

        for _i in 0..size{
            let elem = stack_vec[first_index].pop().unwrap();
            stack_vec[second_index].insert(insert_index, elem);

        }
    }

    let mut result:Vec<char> = vec![];
    for x in stack_vec {
        let temp = x[x.len()-1];
        result.push(temp);
    }

    for elem in result {
        print!("{}", elem);
    }

}