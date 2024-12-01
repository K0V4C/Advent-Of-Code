use std::fs;

static NUMPAD: [[i32; 3]; 3] = [
    [1, 2 ,3],
    [4, 5, 6],
    [7, 8, 9]
];

static ALIEN_NUMPAD: [[i32; 5]; 5] =[
    [0, 0, 1, 0, 0],
    [0, 2, 3, 4, 0],
    [5, 6, 7, 8, 9],
    [0, 10,11,12,0],
    [0, 0, 13, 0, 0]
];

fn number(line: &str, start: (i32, i32)) -> (i32, (i32, i32)) {
    let (mut x,mut  y) = start;

    // println!("START:{:?}", (x, y));

    for e in line.chars() {

        match e {
            'U' => x = if x == 0 { 0 } else { x - 1 },
            'R' => y = if y == 2 { 2 } else { y + 1 },
            'D' => x = if x == 2 { 2 } else { x + 1 },
            'L' => y = if y == 0 { 0 } else { y - 1 },
            _ => (),
        };
        // println!("{} -> {x}{y}", NUMPAD[x as usize][y as usize]);
    }



    (NUMPAD[x as usize][y as usize], (x,y))
}

fn number_alien(line: &str, start: (i32, i32)) -> (i32, (i32, i32)) {

    let (mut x, mut y) = start;

    // println!("START:{:?}", (x, y));

    for e in line.chars() {

        match e {
            'U' => {
                x = if x == 0 { 0 } else { x - 1 };

                if ALIEN_NUMPAD[x as usize][y as usize] == 0{
                    x += 1;
                }

            },
            'R' => {
                y = if y == 4 { 4 } else { y + 1 };

                if ALIEN_NUMPAD[x as usize][y as usize] == 0 {
                    y -= 1;
                }
            },
            'D' => {
                x = if x == 4 { 4 } else { x + 1 };

                if ALIEN_NUMPAD[x as usize][y as usize] == 0 {
                    x -= 1;
                }
            },
            'L' => {
                y = if y == 0 { 0 } else { y - 1 };

                if ALIEN_NUMPAD[x as usize][y as usize] == 0 {
                    y += 1;
                }
            },
            _ => (),
        };

        // println!("{} -> {x}{y}", ALIEN_NUMPAD[x as usize][y as usize]);
    }


    (ALIEN_NUMPAD[x as usize][y as usize], (x,y))
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("File doesn't exist");
    let input = input.trim().split("\n").collect::<Vec<_>>();

    let mut code: i32 = 0;
    let mut start = (1, 1);

    for line in &input {
        let (num, tmp) = number(&line, start);
        start.0 = tmp.0;
        start.1 = tmp.1;
        code = (code + num) * 10;
    }

    println!("{:?}", code/10);

    let mut code: i64 = 0;

    start = (2, 0);

    for line in input{
        let (num, tmp) = number_alien(&line, start);
        start = tmp;
        code = (code + num as i64 ) * 16;
    }

    println!("{:#02X}", code/16);
}
