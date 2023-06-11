use core::fmt;
use std::{cmp::min, collections::HashSet, path::Display, vec};

#[derive(Debug, PartialEq, Default, Clone)]
struct State {
    chps: Vec<i32>,
    gens: Vec<i32>,
    tbl: Box<Vec<char>>,
    elevator: u32,
    iterations: u32,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: String = "".to_string();
        for i in (0..4).rev() {
            for j in 0..self.tbl.len() {
                if self.chps[j] == i {
                    res = format!("{res} {}M ", self.tbl[j]);
                } else {
                    res = format!("{res} .  ");
                }
            }
            for j in 0..self.tbl.len() {
                if self.gens[j] == i {
                    res = format!("{res} {}G ", self.tbl[j]);
                } else {
                    res = format!("{res} .  ");
                }
            }
            res = format!("{res}\n");
        }
        write!(f, "{res}")
    }
}

fn input_ex() -> State {
    State {
        chps: vec![0, 2],
        gens: vec![0, 2],
        tbl: Box::new(vec!['H', 'L']),
        elevator: 0,
        iterations: 0,
    }
}
fn is_legal_state(s: State) -> bool {
    for i in 0..4 {
        let gens: HashSet<_> = s.gens.iter().filter(|x| **x == i).enumerate().collect();
        let chip: HashSet<_> = s.chps.iter().filter(|x| **x == i).enumerate().collect();
        let diff = gens.difference(&chip).count();
        let diff1 = chip.difference(&gens).count();
        if diff > 0 && diff1 > 0 {
            return false;
        }
    }
    true
}

fn is_win(s: State) -> bool {
    if s.chps.iter().min() == Some(&3) && s.gens.iter().min() == Some(&3) {
        return true;
    }
    false
}
fn heuristic(s: State) -> i32 {
    let mut res: i32 = 0;
    for c in s.chps.iter() {
        res += 3 - c;
    }
    for g in s.gens.iter() {
        res += 3 - g;
    }
    res
}

fn generate_next_floor(s: &State, floor: usize, dir: i32) -> Vec<State> {
    let mut res = vec![];
    let gen_moves:Vec<_> = s
        .gens
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&x| x.1 as usize == floor)
        .map(|x| (x.0, x.1 + dir)).collect();
    let chp_moves:Vec<_> = s
        .chps
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&x| x.1 as usize == floor)
        .map(|x| (x.0, x.1 + dir)).collect();

    // move one or 2 chips
    for i in 0..chp_moves.len() {
        for j in i..chp_moves.len() {
            if j == i {
                res.push(State { chps: (), gens: (), tbl: s.tbl , elevator: floor+dir, iterations: () })


            } else {


            }
        }
    }

    res
}

fn generate_moves(s: State, floor: usize) -> Vec<State> {
    let mut res = vec![];
    if floor < 3 {
        res.append(&mut generate_next_floor(&s, floor + 1, 1));
    }
    if floor > 0 {
        res.append(&mut generate_next_floor(&s, floor - 1, -1));
    }
    res
}

fn Part1() -> u32 {
    let init = input_ex();
    println!("{}", is_legal_state(init));
    0
}

fn main() {
    // println!("Part 1 {}", Part1());
    println!("Part 1 {}", Part1());
}
