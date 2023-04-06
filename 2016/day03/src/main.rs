use std::{array, fs, num, str::FromStr, usize};

fn is_triangle_vec(s: &Vec<i32>) -> bool {
    return (s[0] < s[1] + s[2]) && (s[1] < s[2] + s[0]) && (s[2] < s[1] + s[0]);
}

fn Part1(INPUT_FILE: &str) -> i32 {
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let mut count = 0;

    for s in contents.lines() {
        let nums: Vec<i32> = s
            .split(" ")
            .filter(|&x| x.len() != 0)
            .map(from_str)
            .collect();
        if is_triangle_vec(&nums) {
            count += 1;
        }
    }
    return count;
}

fn is_triangle(s0: i32, s1: i32, s2: i32) -> bool {
    return (s0 < s1 + s2) && (s1 < s2 + s0) && (s2 < s1 + s0);
}

fn Part2(INPUT_FILE: &str) -> i32 {
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let mut count = 0;
    let mut trips: Vec<Vec<i32>> = vec![];

    for s in contents.lines() {
        let nums: Vec<i32> = s
            .split(" ")
            .filter(|&x| x.len() != 0)
            .map(from_str)
            .collect();
        trips.push(nums);
        if trips.len() == 3 {
            for i in 0..3 {
                if is_triangle(trips[0][i], trips[1][i], trips[2][i]) {
                    count += 1;
                }
            }
            trips.clear();
        }
    }
    return count;
}

fn from_str(i: &str) -> i32 {
    let mut res: i32 = 0;
    for j in i.chars() {
        match j {
            '0'..='9' => res = res * 10 + (j.to_digit(10).unwrap()) as i32,
            _ => (),
        }
    }
    res
}

fn main() {
    const INPUT_FILE: &str = "./input";
    println!("Part 1:{}", Part1(INPUT_FILE));
    println!("Part 2:{}", Part2(INPUT_FILE));
}
