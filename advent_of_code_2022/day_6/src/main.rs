fn all_unique(vec: &Vec<char>) -> bool {
    for (i, x) in vec.iter().enumerate() {
        for (j, y) in vec.iter().enumerate() {
            if x == y && i != j {
                return false;
            }
        }
    }
    return true;
}

fn find_marker(data: &String, box_size: usize) -> i32 {
    let mut char_box: Vec<char> = vec![];
    for (i, x) in data.chars().enumerate() {
        if i < box_size {
            char_box.push(x);
            continue;
        }

        if char_box.len() < box_size {
            break;
        }

        if char_box.len() == 4 {
            return i as i32;
        }
        println!("{:?}", char_box);
        char_box.remove(0);
        char_box.push(x);
    }
    return -1;
}

fn main() {
    let data = std::fs::read_to_string("data.txt").unwrap();

    let index = find_marker(&data, 4);
    println!("{}", index);

    let index = find_marker(&data, 14);
    println!("{}", index);
}
