use std::{fs, io, mem::swap};

use itertools::Itertools;

fn index_of(work: &Vec<char>, ch: char) -> usize {
    for (i, &c) in work.into_iter().enumerate() {
        if c == ch {
            return i;
        }
    }
    work.len()
}

fn compute_instruction_part1(work: &mut Vec<char>, instruction: &str) {
    let cmd: Vec<_> = instruction.split(" ").collect();
    match cmd[0] {
        "swap" => {
            let mut i1: usize = 0;
            let mut i2: usize = 0;
            if cmd[1] == "letter" {
                i1 = index_of(
                    &work,
                    cmd[2].chars().nth(0).expect("string shouldnt be empty"),
                );
                i2 = index_of(
                    &work,
                    cmd[5].chars().nth(0).expect("string shouldnt be empty"),
                );
            } else {
                i1 = cmd[2].parse::<usize>().expect("expected a number");
                i2 = cmd[5].parse::<usize>().expect("expected a number");
            }
            // rotate
            let p = work[i1];
            work[i1] = work[i2];
            work[i2] = p;
        }
        "rotate" => {
            let mut i1: i32 = 0;
            let mut direction = 1;
            match cmd[1] {
                "based" => {
                    i1 = index_of(
                        &work,
                        cmd[6].chars().nth(0).expect("string shouldnt be empty"),
                    ) as i32;
                    if i1 > 3 {
                        i1 += 1
                    }
                    i1 += 1
                }
                "right" => {
                    i1 = cmd[2].parse::<i32>().expect("expected a number");
                }
                "left" => {
                    i1 = cmd[2].parse::<i32>().expect("expected a number");
                    direction = -1
                }
                _ => (),
            }
            // rotate
            i1 = ((i1 * direction + work.len() as i32) % work.len() as i32);
            let work_tmp = work.to_owned();
            for i in 0..work_tmp.len() {
                let idx: usize = (i + i1 as usize) % work.len();
                work[idx] = work_tmp[i];
            }
        }
        "move" => {
            let mut i1 = cmd[2].parse::<usize>().expect("expected a number");
            let mut i2 = cmd[5].parse::<usize>().expect("expected a number");
            let store = work.remove(i1);
            work.insert(i2, store);
        }
        "reverse" => {
            let mut i1 = cmd[2].parse::<usize>().expect("expected a number");
            let mut i2 = cmd[4].parse::<usize>().expect("expected a number");
            work[i1..i2 + 1].reverse();
        }
        _ => (),
    }
}
fn simulate_move_as_result(cnt_idx: usize, ln: usize) -> usize {
    for i in 0..ln {
        let mut tmp_pos = 1 + i;
        if i > 4 {
            tmp_pos += 1;
        }
        if tmp_pos % ln == cnt_idx {
            return i;
        }
    }
    0
}

fn compute_instruction_part2(work: &mut Vec<char>, instruction: &str) {
    let cmd: Vec<_> = instruction.split(" ").collect();
    match cmd[0] {
        "swap" => {
            let mut i1: usize = 0;
            let mut i2: usize = 0;
            if cmd[1] == "letter" {
                i1 = index_of(
                    &work,
                    cmd[2].chars().nth(0).expect("string shouldnt be empty"),
                );
                i2 = index_of(
                    &work,
                    cmd[5].chars().nth(0).expect("string shouldnt be empty"),
                );
            } else {
                i1 = cmd[2].parse::<usize>().expect("expected a number");
                i2 = cmd[5].parse::<usize>().expect("expected a number");
            }
            // rotate
            let p = work[i1];
            work[i1] = work[i2];
            work[i2] = p;
        }
        "rotate" => {
            let mut i1: i32 = 0;
            let mut direction = 1;
            match cmd[1] {
                "based" => {
                    let crnt_pos = index_of(
                        &work,
                        cmd[6].chars().nth(0).expect("string shouldnt be empty"),
                    );
                    i1 = simulate_move_as_result(crnt_pos, work.len()) as i32;
                }
                "right" => {
                    i1 = ((work.len() - cmd[2].parse::<usize>().expect("expected a number"))
                        % work.len()) as i32;
                }
                "left" => {
                    i1 = ((work.len() - cmd[2].parse::<usize>().expect("expected a number"))
                        % work.len()) as i32;
                    direction = -1
                }
                _ => (),
            }
            // rotate
            i1 = ((i1 * direction + work.len() as i32) % work.len() as i32);
            let work_tmp = work.to_owned();
            for i in 0..work_tmp.len() {
                let idx: usize = (i + i1 as usize) % work.len();
                work[idx] = work_tmp[i];
            }
        }
        "move" => {
            let mut i2 = cmd[2].parse::<usize>().expect("expected a number");
            let mut i1 = cmd[5].parse::<usize>().expect("expected a number");
            let store = work.remove(i1);
            work.insert(i2, store);
        }
        "reverse" => {
            // println!("{:?}",cmd);
            let mut i1 = cmd[2].parse::<usize>().expect("expected a number");
            let mut i2 = cmd[4].parse::<usize>().expect("expected a number");
            work[i1..i2 + 1].reverse();
        }
        _ => (),
    }
}

fn part1(input_file: &str, seed: &str) -> String {
    let mut work: Vec<char> = seed.chars().collect();
    let inp = fs::read_to_string(input_file).expect("File not exist");
    for l in inp.lines() {
        compute_instruction_part1(&mut work, l);
    }

    work.into_iter().collect()
}

fn part2_brute_froce(input_file: &str, seed: &str) -> String {
    let translate = ['a','b','c','d','e','f','g','h'];
    for i in (0..8).permutations(8) {
        let perm_seed = i.into_iter().map(|x| translate[x]).collect::<String>();
        if seed == part1(input_file,&perm_seed) {
            return perm_seed;
        }
    }
    return "".to_string();
}

fn main() {
    // println!("{}", part1("./input", "abcdefgh"));
    println!("{}", part2_brute_froce("./input", "fbgdceah"));
    println!("{}", part2_brute_froce("./input1", "decab"));
    // println!("{}", part1("./input1", "abcde"));
}
