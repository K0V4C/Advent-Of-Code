/*

    Solution for Advent of Code Day 4

*/

use std::iter::Enumerate;

use regex::Regex;

pub fn run() {
    println!("--- Started solving day 4 ---");

    let _test_file_path = "./src/day_4/test.txt";
    let _problem_file_path = "./src/day_4/problem.txt";

    let read_file = std::fs::read_to_string(_problem_file_path).unwrap();

    let lines = read_file.lines().collect::<Vec<&str>>();

    let width = lines[0].len();
    let hight = lines.len();

    let mut vertical_lines: Vec<String> = vec![String::new(); width];
    let mut horizontal_lines: Vec<String> = vec![];
    let mut diagonal_lines_right: Vec<String> = vec![];
    let mut diagonal_lines_left: Vec<String> = vec![];

    // Prepare data

    horizontal_lines = read_file.lines().map(|x| String::from(x)).collect();

    for one_line in horizontal_lines.iter() {
        let mut idx = 0;
        for c in one_line.chars() {
            vertical_lines[idx].push(c);
            idx += 1;
        }
    }

    let mut byte_matrix: Vec<Vec<u8>> = vec![vec![0; width]; hight];

    for (i, one_line) in horizontal_lines.iter().enumerate() {
        let mut j = 0;
        for b in one_line.as_bytes() {
            byte_matrix[i][j] = *b;
            j += 1;
        }
    }

    let mut i = 0;
    let mut j = 0;
    loop {
        let mut byte_vec = vec![];
        let mut k = i;
        let mut m = j;
        loop {
            byte_vec.push(byte_matrix[k][m]);

            if k == 0 || m == width - 1 {
                break;
            }

            k -= 1;
            m += 1;
        }

        diagonal_lines_right.push(String::from_utf8(byte_vec).expect("This vector wasnt utf8"));

        if i < hight - 1 {
            i += 1;
        } else {
            j += 1;
        }

        if j == width {
            break;
        }
    }

    let mut i = 0;
    let mut j = width - 1;
    loop {
        let mut byte_vec = vec![];
        let mut k = i;
        let mut m = j;
        loop {
            byte_vec.push(byte_matrix[k][m]);

            if k == 0 || m == 0 {
                break;
            }

            k -= 1;
            m -= 1;
        }

        diagonal_lines_left.push(String::from_utf8(byte_vec).expect("This vector wasnt utf8"));

        if j == 0 {
            break;
        }

        if i < hight - 1 {
            i += 1;
        } else {
            j -= 1;
        }
    }

    // Check with regex

    let search_regex = Regex::new(r"XMAS").unwrap();
    let reverse_regex = Regex::new(r"SAMX").unwrap();

    let mut all_lines: Vec<String> = vec![];
    all_lines.extend(vertical_lines);
    all_lines.extend(horizontal_lines);
    all_lines.extend(diagonal_lines_right);
    all_lines.extend(diagonal_lines_left);

    let total_result = all_lines.iter().fold(0, |acc, line| {
        acc + search_regex.find_iter(line).count() + reverse_regex.find_iter(line).count()
    });

    println!("SOLUTION FOR PART 1 >>>>> {}", total_result);

    println!("--- Started solving day 4 part 2 ---");

    let read_file = std::fs::read_to_string(_problem_file_path).unwrap();

    let lines = read_file.lines().collect::<Vec<&str>>();

    let width = lines[0].len();
    let hight = lines.len();

    let mut byte_matrix: Vec<Vec<u8>> = vec![vec![0; width]; hight];

    let horizontal_lines: Vec<String> = read_file.lines().map(|x| String::from(x)).collect();

    for (i, one_line) in horizontal_lines.iter().enumerate() {
        let mut j = 0;
        for b in one_line.as_bytes() {
            byte_matrix[i][j] = *b;
            j += 1;
        }
    }

    let mut cross_total = 0;

    for i in 0..hight - 2 {
        for j in 0..width - 2 {
            // Check value of this window

            let M = "M".as_bytes()[0];
            let A = "A".as_bytes()[0];
            let S = "S".as_bytes()[0];

            if (byte_matrix[i][j] == M
                && byte_matrix[i + 1][j + 1] == A
                && byte_matrix[i + 2][j + 2] == S
                && byte_matrix[i][j + 2] == S
                && byte_matrix[i + 2][j] == M)
                || (byte_matrix[i][j] == M
                    && byte_matrix[i + 1][j + 1] == A
                    && byte_matrix[i + 2][j + 2] == S
                    && byte_matrix[i][j + 2] == M
                    && byte_matrix[i + 2][j] == S)
                || (byte_matrix[i][j] == S
                    && byte_matrix[i + 1][j + 1] == A
                    && byte_matrix[i + 2][j + 2] == M
                    && byte_matrix[i][j + 2] == S
                    && byte_matrix[i + 2][j] == M)
                || (byte_matrix[i][j] == S
                    && byte_matrix[i + 1][j + 1] == A
                    && byte_matrix[i + 2][j + 2] == M
                    && byte_matrix[i][j + 2] == M
                    && byte_matrix[i + 2][j] == S)
            {
                cross_total += 1;
            }
        }
    }

    println!("CROSS TOTAL >>>>>  {}", cross_total);

    println!("--- All done! ---");
}
