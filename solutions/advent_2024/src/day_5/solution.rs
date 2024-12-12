/*

    Solution for Advent of Code Day 5

*/

use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub fn run() {
    println!("--- Started solving day 5 ---");

    let _test_file_path = "./src/day_5/test.txt";
    let _problem_file_path = "./src/day_5/problem.txt";

    let read_file = std::fs::read_to_string(_problem_file_path).unwrap();

    let mut rule_book: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut rules: Vec<Vec<i32>> = vec![];

    for line in read_file.lines() {
        match line {
            x if line.contains('|') => {
                let split_input = x
                    .trim()
                    .split('|')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();

                rule_book
                    .entry(split_input[1])
                    .and_modify(|x| {
                        x.insert(split_input[0]);
                    })
                    .or_insert_with(|| {
                        let mut h = HashSet::new();
                        h.insert(split_input[0]);
                        return h;
                    });
            }
            x if line.contains(',') => {
                let split_input = x
                    .trim()
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();

                rules.push(split_input);
            }
            _ => {}
        }
    }

    let mut total_score = 0;

    let mut incorrect_rules: Vec<Vec<i32>> = vec![];

    'outer: for rule in rules {
        let mut printed_pages: HashSet<i32> = HashSet::new();

        let rule_pages: HashSet<i32> = rule.clone().into_iter().collect();

        for page in &rule {
            let rule_for_page = if let Some(val) = rule_book.get(&page) {
                val
            } else {
                rule_book.insert(*page, HashSet::new());
                rule_book.get(&page).unwrap()
            };

            let rule_for_page: HashSet<i32> =
                rule_pages.intersection(rule_for_page).cloned().collect();

            if rule_for_page.is_subset(&printed_pages) == false {
                incorrect_rules.push(rule);
                continue 'outer;
            }

            printed_pages.insert(*page);
        }

        let middle_point = rule.len() / 2;

        total_score += rule[middle_point];
    }

    println!("SOLUTION FOR PART 1 >>>> {}", total_score);

    println!("--- Started solving day 5 part 2 ---");

    let mut total_score = 0;

    for incorrect_rule in incorrect_rules {
        let mut correctly_sorted_rule: Vec<i32> = vec![];
        let mut printed_pages: HashSet<i32> = HashSet::new();
        let rule_pages: HashSet<i32> = incorrect_rule.clone().into_iter().collect();
        // iterate thrugh the rule until properly sorted
        loop {
            for page in &incorrect_rule {
                let rule_for_page = if let Some(val) = rule_book.get(&page) {
                    val
                } else {
                    rule_book.insert(*page, HashSet::new());
                    rule_book.get(&page).unwrap()
                };

                let rule_for_page: HashSet<i32> =
                    rule_pages.intersection(rule_for_page).cloned().collect();

                if rule_for_page.is_subset(&printed_pages) == false {
                    continue;
                }

                if printed_pages.contains(page) == false {
                    correctly_sorted_rule.push(*page);
                }
                printed_pages.insert(*page);
            }

            if correctly_sorted_rule.len() == incorrect_rule.len() {
                // Sorted perfectly
                total_score += correctly_sorted_rule[correctly_sorted_rule.len() / 2];
                break;
            }
        }
    }

    println!("Solution for PART 2 >>>>> {}", total_score);

    println!("--- All done! ---");
}
