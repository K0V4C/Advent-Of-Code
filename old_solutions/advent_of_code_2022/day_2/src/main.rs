use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt");
    let data = match data {
        Ok(data) => data,
        Err(error) => panic!("File not found: {}", error),
    };

    let data = data.split("\n");

    let mut score = 0;

    for x in data{

        let line:Vec<&str> = x.split(" ").collect();
        
        match line[0] {
            "A" => {
                match line[1] {
                    "X" => {
                        score += 1;
                        score += 3;
                    },
                    "Y" => {
                        score += 2;
                        score += 6;
                    },
                    "Z" => {
                        score += 0;
                        score += 3;
                    },
                    _ => panic!("This houldnt happen ( in match ) {}" , line[1])
                };

            },
            "B" => {

                match line[1] {
                    "X" => {
                        score += 0;
                        score += 1;
                    },
                    "Y" => {
                        score += 2;
                        score += 3;
                    },
                    "Z" => {
                        score +=3;
                        score +=6;
                    },
                    _ => panic!("This houldnt happen ( in match ) {:?}", line[1])
                };

            },
            "C" => {
                match line[1] {
                    "X" => {
                        score +=1;
                        score +=6;
                    },
                    "Y" => {
                        score +=2;
                    },
                    "Z" => {
                        score +=3;
                        score +=3;
                    },
                    _ => panic!("This houldnt happen ( in match ) {:?}", line[1])
                };

            },
            _ => panic!("This shouldn't happen")
        };
    }

    println!("This is the result: {}", score);



    let new_data = fs::read_to_string("data.txt");
    let new_data = match new_data {
        Ok(new_data) => new_data,
        Err(error) => panic!("Reading file failed {}", error)
    };
    let new_data = new_data.split("\n");

    score = 0;

    for x in new_data{
        let line:Vec<&str> = x.split(" ").collect();

        match line[0] {
            "A" => {
                match line[1] {
                    "X" => {
                        score += 0;
                        score += 3;
                    },
                    "Y" => {
                        score += 1;
                        score += 3;
                    },
                    "Z" => {
                        score += 2;
                        score += 6;
                    },
                    _ => panic!("This houldnt happen ( in match ) {}" , line[1])
                };

            },
            "B" => {

                match line[1] {
                    "X" => {
                        score += 0;
                        score += 1;
                    },
                    "Y" => {
                        score += 2;
                        score += 3;
                    },
                    "Z" => {
                        score +=3;
                        score +=6;
                    },
                    _ => panic!("This houldnt happen ( in match ) {:?}", line[1])
                };

            },
            "C" => {
                match line[1] {
                    "X" => {
                        score +=2;
                        score +=0;
                    },
                    "Y" => {
                        score +=3;
                        score +=3;
                    },
                    "Z" => {
                        score +=1;
                        score +=6;
                    },
                    _ => panic!("This houldnt happen ( in match ) {:?}", line[1])
                };

            },
            _ => panic!("This shouldn't happen")
        };
    }

    print!("New guide: {}", score);


}
