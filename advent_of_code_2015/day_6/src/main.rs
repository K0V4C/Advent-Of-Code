use std::fs;

pub fn run() {
    let data = fs::read_to_string("data_6th.text").unwrap();
    let data = data.split('\n').collect::<Vec<&str>>();
    let mut lights = vec![vec![0; 1000]; 1000];
    for x in data {
        let temp = x; //let binding
        let words = temp.split(' ').collect::<Vec<&str>>();

        let last_index = words.len() - 1;
        let first_number_pair = words[last_index - 2]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        let mut second_number_pair = words[last_index]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();

        second_number_pair[0] += 1;
        second_number_pair[1] += 1;

        let command = words[..last_index - 2];
        let command = command.join("");
        //println!("{:?}{:?}{:?}", first_number_pair, second_number_pair, command);

        match command.as_str() {
            "toggle" => {
                for i in first_number_pair[0]..second_number_pair[0] {
                    for j in first_number_pair[1]..second_number_pair[1] {
                        // if lights[i][j] {true} else {false};
                        lights[i][j] += 2;
                    }
                }
            }
            "turnoff" => {
                for i in first_number_pair[0]..second_number_pair[0] {
                    for j in first_number_pair[1]..second_number_pair[1] {
                        // lights[i][j] = false;
                        if lights[i][j] != 0 {
                            lights[i][j] -= 1;
                        }
                    }
                }
            }
            "turnon" => {
                for i in first_number_pair[0]..second_number_pair[0] {
                    for j in first_number_pair[1]..second_number_pair[1] {
                        // lights[i][j] = true;
                        lights[i][j] += 1;
                    }
                }
            }
            _ => {}
        }
    }

    let mut num: u32 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            num += lights[i][j] as u32;
        }
    }

    println!("{}", num);
}
