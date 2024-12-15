/*

    Solution for Advent of Code Day 6

*/

use std::collections::HashSet;

struct Grid {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input_str: String) -> Self {
        let tiles: Vec<Vec<char>> = input_str
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let width = tiles[0].len();
        let height = tiles.len();

        Grid {
            tiles,
            width,
            height,
        }
    }

    fn get_guard_position(&self) -> (usize, usize) {
        for row in 0..self.width {
            for col in 0..self.height {
                if self.tiles[row][col] == '^' {
                    return (row, col);
                }
            }
        }

        panic!("No guard found");
    }

    fn get_next_position(
        &self,
        (guard_row, guard_col): (usize, usize),
        direction: &mut Direction,
    ) -> Option<(usize, usize)> {
        let (next_row, next_col) = match direction {
            Direction::Up => (guard_row.checked_sub(1)?, guard_col),
            Direction::Right => (guard_row, guard_col + 1),
            Direction::Down => (guard_row + 1, guard_col),
            Direction::Left => (guard_row, guard_col.checked_sub(1)?),
        };

        let char = self.tiles.get(next_row).and_then(|row| row.get(next_col))?;

        if char == &'#' {
            direction.turn_right();
            return Some((guard_row, guard_col));
        }

        Some((next_row, next_col))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

fn gets_in_loop(
    grid: &Grid,
    (start_row, start_col): (usize, usize),
    start_direction: Direction,
) -> bool {
    // only need to keep track of the times the guard turned
    // if the guard made the same turn at the same obstacle twice then we have a cycle
    let mut visited_obstacles: Vec<(usize, usize, Direction)> = Vec::new();

    let mut direction = start_direction;
    let (mut guard_row, mut guard_col) = (start_row, start_col);

    while let Some((next_row, next_col)) =
        grid.get_next_position((guard_row, guard_col), &mut direction)
    {
        if (guard_row, guard_col) == (next_row, next_col) {
            if visited_obstacles.contains(&(guard_row, guard_col, direction)) {
                return true;
            }

            visited_obstacles.push((guard_row, guard_col, direction));
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    false
}

pub fn run() {
    println!("--- Started solving day 6 ---");

    let _test_file_path = "./src/day_6/test.txt";
    let _problem_file_path = "./src/day_6/problem.txt";

    let input = std::fs::read_to_string(_problem_file_path).unwrap();

    let grid = Grid::new(input.clone());
    let (mut guard_row, mut guard_col) = grid.get_guard_position();
    let mut direction = Direction::Up;

    let mut visited = HashSet::new();
    visited.insert((guard_row, guard_col));

    while let Some((next_row, next_col)) =
        grid.get_next_position((guard_row, guard_col), &mut direction)
    {
        guard_row = next_row;
        guard_col = next_col;

        visited.insert((guard_row, guard_col));
    }

    println!("Number of squares >>> {}", visited.len());

    println!("--- Started solving day 6 part 2 ---");

    let mut grid = Grid::new(input);
    let (mut guard_row, mut guard_col) = grid.get_guard_position();
    let mut direction = Direction::Up;

    let mut visited = HashSet::new();
    let mut count = 0;

    while let Some((next_row, next_col)) =
        grid.get_next_position((guard_row, guard_col), &mut direction)
    {
        visited.insert((guard_row, guard_col));

        if !visited.contains(&(next_row, next_col)) {
            grid.tiles[next_row][next_col] = '#';
            if gets_in_loop(&grid, (guard_row, guard_col), direction) {
                count += 1;
            }
            grid.tiles[next_row][next_col] = '.';
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    println!("Solution for PART 2 >>> {}", count);

    println!("--- All done! ---");
}
