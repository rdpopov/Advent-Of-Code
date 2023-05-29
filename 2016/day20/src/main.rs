use std::{fs, cmp::max};

fn Part1(INPUT_FILE:&str) -> u64 {
    let inp = fs::read_to_string(INPUT_FILE).expect("Sohuld be able to read file");
    let mut ranges:Vec<(u64,u64)> = vec![(0,0);inp.lines().count()];
    for (idx,line) in inp.lines().enumerate() {
        let i = line.split('-').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        ranges[idx] = (i[0],i[1]);
    }
    ranges.sort();
    let mut crnt_idx:usize = 0;
    loop {
        if crnt_idx+1 == ranges.len() {
            break;
        }
        if ranges[crnt_idx].1 >= (ranges[crnt_idx+1].0-1) {
            ranges[crnt_idx].1  = max(ranges[crnt_idx].1,ranges[crnt_idx+1].1);
            ranges.remove(crnt_idx+1);
        } else {
            crnt_idx+=1;
        }
    }
    ranges[0].1 + 1
}

fn Part2(INPUT_FILE:&str) -> u64 {
    let inp = fs::read_to_string(INPUT_FILE).expect("Sohuld be able to read file");
    let mut ranges:Vec<(u64,u64)> = vec![(0,0);inp.lines().count()];
    let mut result = 0;
    for (idx,line) in inp.lines().enumerate() {
        let i = line.split('-').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        ranges[idx] = (i[0],i[1]);
    }
    ranges.sort();
    let mut crnt_idx:usize = 0;
    loop {
        if crnt_idx+1 == ranges.len() {
            break;
        }
        if ranges[crnt_idx].1 >= (ranges[crnt_idx+1].0-1) {
            ranges[crnt_idx].1  = max(ranges[crnt_idx].1,ranges[crnt_idx+1].1);
            ranges.remove(crnt_idx+1);
        } else {
            crnt_idx+=1;
        }
    }
    for idx in 0..ranges.len()-1 {
        result += ranges[idx+1].0 - ranges[idx].1 - 1;
    }
    result
}
fn main() {
    println!("{}",Part1("./input"));
    println!("{}",Part2("./input"));
}
