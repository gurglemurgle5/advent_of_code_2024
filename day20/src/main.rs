use core::num;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (mut start_row, mut start_col) = (0, 0);
    let (mut end_row, mut end_col) = (0, 0);
    for (row, line) in map.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            match *char {
                'S' => (start_row, start_col) = (row, col),
                'E' => (end_row, end_col) = (row, col),
                _ => (),
            }
        }
    }
    let baseline = a_star(
        &map,
        (start_row, start_col),
        (end_row, end_col),
        |(row, col)| {
            0 // lazy
        },
    )
    .len();
    dbg!(baseline);

    let mut num_time_saving_cheats = 0;

    for row in 0..map.len() {
        println!("doing row {row}");
        for col in 0..map[row].len() {
            if map[row][col] != '#' {
                continue;
            }
            // for every position, there are two potential cheats: right, and down
            // oh goodness i sure hope they don't allow cheating for longer time periods
            // lemee guess that's part 2
            match map[row][col] {
                '.' | 'S' | 'E' => map[row][col] = 'T',
                '#' => map[row][col] = 'W',
                _ => panic!(),
            }
            let test_score = a_star(
                &map,
                (start_row, start_col),
                (end_row, end_col),
                |(row, col)| {
                    0 // lazy
                },
            )
            .len();
            if test_score + 100 <= baseline {
                num_time_saving_cheats += 1;
            }

            match map[row][col] {
                'T' => map[row][col] = '.',
                'W' => map[row][col] = '#',
                _ => panic!(),
            }
        }
    }
    dbg!(num_time_saving_cheats);
}

type Node = (usize, usize);

fn reconstruct_path(came_from: HashMap<Node, Node>, mut current: Node) -> Vec<Node> {
    let mut total_path = Vec::new();
    // let mut score = 0;
    while let Some(other) = came_from.get(&current) {
        // score += d(current, *other);
        current = *other;
        total_path.push(current);
    }
    total_path.reverse();
    // score
    total_path
}

fn a_star<T: FnMut(Node) -> usize>(
    input: &[Vec<char>],
    start: Node,
    goal: Node,
    mut h: T,
) -> Vec<Node> {
    let mut open_set = HashSet::new();
    open_set.insert(start);
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score = HashMap::new();
    f_score.insert(start, h(start));

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by_key(|node| f_score.get(node).unwrap_or(&usize::MAX))
            .unwrap();
        if current == goal {
            return reconstruct_path(came_from, current);
        }
        open_set.remove(&current);
        for neighbor in generate_neighbors(input, current) {
            let tentative_g_score = g_score.get(&current).unwrap() + d(current, neighbor);
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(neighbor));
                open_set.insert(neighbor);
            }
        }
    }
    panic!("failed to reach goal!");
}

/// PRECONDITION: current and neighbor are direct neighbors
fn d(_current: (usize, usize), _neighbor: (usize, usize)) -> usize {
    1
}

fn generate_neighbors(input: &[Vec<char>], current: Node) -> Vec<Node> {
    let mut neighbors = Vec::new();

    let (row, col) = current;

    if row + 1 < input.len() && input[row + 1][col] != '#' {
        neighbors.push((row + 1, col));
    }
    if row > 0 && input[row - 1][col] != '#' {
        neighbors.push((row - 1, col));
    }
    if col + 1 < input[row].len() && input[row][col + 1] != '#' {
        neighbors.push((row, col + 1));
    }
    if col > 0 && input[row][col - 1] != '#' {
        neighbors.push((row, col - 1));
    }

    neighbors
}
