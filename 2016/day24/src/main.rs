use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    vec,
};

use priority_queue::PriorityQueue;

type Point = (i64, i64);
fn a_star(board: &Vec<Vec<char>>, start: char, end: char) -> i64 {
    let start = find_char(board, start).expect(format!("Unknown char {}", start).as_str());
    let end = find_char(board, end).expect(format!("Unknown char {}", end).as_str());
    let mut been_to: HashMap<Point, i64> = HashMap::new();
    let mut next: PriorityQueue<Point, i64> = PriorityQueue::new();
    let mut chain: HashMap<Point, Point> = HashMap::new();
    next.push(start, 0);

    while !next.is_empty() {
        let (crnt, dist) = next.pop().unwrap();
        if crnt == end {
            return -dist;
        }
        for new_square in generate_neigh(board, crnt) {
            let sq_priority = dist - 1;
            if sq_priority > *been_to.get(&new_square).unwrap_or(&i64::MIN) {
                been_to.insert(new_square, sq_priority);
                next.push(new_square, sq_priority);
                chain.insert(new_square, crnt);
            }
        }
    }
    -1
}

fn find_char(board: &Vec<Vec<char>>, start: char) -> Option<(i64, i64)> {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == start {
                return Some((x as i64, y as i64));
            }
        }
    }
    None
}
fn find_targets(board: &Vec<Vec<char>>) -> Vec<char> {
    let mut res = vec![];
    for t in '0'..':' {
        // from 0 to 9
        match find_char(board, t) {
            Some(_) => res.push(t),
            None => (),
        }
    }
    res
}

fn generate_neigh(board: &Vec<Vec<char>>, c: Point) -> Vec<Point> {
    let mut res = vec![];
    // x
    if c.0 > 1 && board[c.1 as usize][(c.0 - 1) as usize] != '#' {
        res.push((c.0 - 1, c.1));
    }
    if (c.0 as usize) < board[0].len() && board[c.1 as usize][(c.0 + 1) as usize] != '#' {
        res.push((c.0 + 1, c.1));
    }
    // y
    if c.1 > 1 && board[(c.1 - 1) as usize][c.0 as usize] != '#' {
        res.push((c.0, c.1 - 1));
    }
    if (c.1 as usize) < board.len() && board[(c.1 + 1) as usize][c.0 as usize] != '#' {
        res.push((c.0, c.1 + 1));
    }
    return res;
}
fn dfs(tbl: &HashMap<(char, char), i64>, start: char, left: &HashSet<char>, cost: i64) -> i64 {
    let mut min_cost = i64::MAX;
    if left.is_empty() {
        return cost;
    }
    for nxt in left.iter() {
        let mut left_after = left.clone();
        let node_cost = tbl.get_key_value(&(start, *nxt)).unwrap().1;
        left_after.remove(nxt);
        min_cost = min(dfs(&tbl, *nxt, &left_after, cost + node_cost), min_cost);
    }
    return min_cost;
}
fn dfs_with_return(
    tbl: &HashMap<(char, char), i64>,
    origin: char,
    start: char,
    left: &HashSet<char>,
    cost: i64,
) -> i64 {
    let mut min_cost = i64::MAX;
    if left.is_empty() {
        return cost + tbl.get_key_value(&(start, origin)).unwrap().1;
    }
    for nxt in left.iter() {
        let mut left_after = left.clone();
        let node_cost = tbl.get_key_value(&(start, *nxt)).unwrap().1;
        left_after.remove(nxt);
        min_cost = min(
            dfs_with_return(&tbl, origin, *nxt, &left_after, cost + node_cost),
            min_cost,
        );
    }
    return min_cost;
}

#[derive(Default)]
struct AlgorithmInputs {
    neighbour_table: HashMap<(char, char), i64>,
    starting_node: char,
    target_set: HashSet<char>,
}

impl AlgorithmInputs {
    fn new(fname: &str) -> Self {
        let mut ret: Self = Self::default();
        let inp: Vec<Vec<char>> = std::fs::read_to_string(fname)
            .expect("Sohuld be able to read file")
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        let targets = find_targets(&inp);
        ret.target_set = targets[1..].iter().map(|x| *x).collect();
        ret.starting_node = targets[0];

        for i in 0..targets.len() {
            let crnt_node = targets[i];
            for j in 0..targets.len() {
                if i != j {
                    let other_node = targets[j];
                    ret.neighbour_table
                        .insert((crnt_node, other_node), a_star(&inp, crnt_node, other_node));
                }
            }
        }
        return ret;
    }
}

fn part1(fname: &str) -> i64 {
    let inp = AlgorithmInputs::new(fname);
    return dfs(&inp.neighbour_table, inp.starting_node, &inp.target_set, 0);
}

fn part2(fname: &str) -> i64 {
    let inp = AlgorithmInputs::new(fname);
    return dfs_with_return(
        &inp.neighbour_table,
        inp.starting_node,
        inp.starting_node,
        &inp.target_set,
        0,
    );
}

fn main() {
    println!("Part 1 {}", part1("./input"));
    println!("Part 2 {}", part2("./input"));
}

#[test]
fn example_part1() {
    assert_eq!(14, part1("./input_example"));
}

#[test]
fn example_part2() {
    assert_eq!(20, part2("./input_example"));
}

#[test]
fn input_part1() {
    assert_eq!(442, part1("./input"));
}

#[test]
fn input_part2() {
    assert_eq!(660, part2("./input"));
}
