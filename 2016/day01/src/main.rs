use std::{array, fs, usize, str::FromStr};

fn Part1(INPUT_FILE: &str) -> (i32,i32){
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let movment_vectors:[(i32,i32);5] = [
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, 0),
        (0, 1),
    ];

    let mut direction: usize = 3; // direction is the index of element in movment_vectors
    let mut position = (0, 0);

    for s in contents.split(',') {
        let i = s.strip_prefix(" ").unwrap_or(s);
        let dir: &str = &i[0..1];
        let intensity:u32 = from_str(&i[1..]);
        match dir {
            "L" => direction = (direction +1) %4,
            "R" => direction = if direction == 0 {3} else {(direction-1) % 4},
            _ =>()
        }
        position.0 = position.0 + movment_vectors[direction].0 * intensity as i32 ;
        position.1 = position.1 + movment_vectors[direction].1 * intensity as i32 ;
    }
    return position;
}

fn from_str(i: &str) -> u32 {
    let mut res: u32 = 0;
    for j in i.chars() {
        match j {
            '0'..='9' => res = res *10 + (j.to_digit(10).unwrap()),
            _ => ()
        }

    }
    res
}


fn main() {
    const INPUT_FILE: &str = "./input";
    Part1(INPUT_FILE);
}

#[test]
fn test1_part1() {
    const INPUT_FILE: &str = "./input1";
    let res = Part1(INPUT_FILE);
    assert!( (res.0 + res.1) == 5);
}

#[test]
fn test2_part1() {
    const INPUT_FILE: &str = "./input2";
    let res = Part1(INPUT_FILE);
    assert!( (res.0 + res.1) == 2);
}

#[test]
fn test5_part1() {
    const INPUT_FILE: &str = "./input3";
    let res = Part1(INPUT_FILE);
    assert!( (res.0 + res.1 )== 12);
}



