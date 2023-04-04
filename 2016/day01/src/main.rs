use std::{array, collections::HashSet, fs, str::FromStr, usize};

fn Part1(INPUT_FILE: &str) -> (i32, i32) {
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let movment_vectors: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut direction: usize = 3; // direction is the index of element in movment_vectors
    let mut position = (0, 0);

    for s in contents.split(',') {
        let i = s.strip_prefix(" ").unwrap_or(s);
        let dir: &str = &i[0..1];
        let intensity: u32 = from_str(&i[1..]);
        match dir {
            "L" => direction = (direction + 1) % 4,
            "R" => {
                direction = if direction == 0 {
                    3
                } else {
                    (direction - 1) % 4
                }
            }
            _ => (),
        }
        position.0 = position.0 + movment_vectors[direction].0 * intensity as i32;
        position.1 = position.1 + movment_vectors[direction].1 * intensity as i32;
    }
    return position;
}

fn from_str(i: &str) -> u32 {
    let mut res: u32 = 0;
    for j in i.chars() {
        match j {
            '0'..='9' => res = res * 10 + (j.to_digit(10).unwrap()),
            _ => (),
        }
    }
    res
}

fn Part2(INPUT_FILE: &str) -> (i32, i32) {
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let movment_vectors: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut direction: usize = 3; // direction is the index of element in movment_vectors
    let mut position = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for s in contents.split(',') {
        let i = s.strip_prefix(" ").unwrap_or(s);
        let dir: &str = &i[0..1];
        let intensity: u32 = from_str(&i[1..]);
        match dir {
            "L" => direction = (direction + 1) % 4,
            "R" => {
                direction = if direction == 0 {
                    3
                } else {
                    (direction - 1) % 4
                }
            }
            _ => (),
        }

        for i in 0..intensity {
            position.0 = position.0 + movment_vectors[direction].0;
            position.1 = position.1 + movment_vectors[direction].1;
            println!("{:?}", &position);
            if visited.insert(position) == false {
                return position;
            }
        }
    }
    return position;
}

fn main() {
    const INPUT_FILE: &str = "./input4";
    println!("{:?}", Part2(INPUT_FILE))
}

mod test {
    use super::*;
    #[test]
    fn test1_part1() {
        const INPUT_FILE: &str = "./input1";
        const expected: i32 = 5;
        let res = Part1(INPUT_FILE);
        let res = (res.0.abs(), res.1.abs());
        let msg = format!("{} != {}", expected, (res.0 + res.1));
        assert!((res.0 + res.1) == expected, "{msg}");
    }

    #[test]
    fn test2_part1() {
        const INPUT_FILE: &str = "./input2";
        const expected: i32 = 2;
        let res = Part1(INPUT_FILE);
        let res = (res.0.abs(), res.1.abs());
        let msg = format!("{} != {}", expected, (res.0 + res.1));
        assert!((res.0 + res.1) == expected, "{msg}");
    }

    #[test]
    fn test3_part1() {
        const INPUT_FILE: &str = "./input3";
        const expected: i32 = 12;
        let res = Part1(INPUT_FILE);
        let res = (res.0.abs(), res.1.abs());
        let msg = format!("{} != {}", expected, (res.0 + res.1));
        assert!((res.0 + res.1) == expected, "{msg}");
    }

    #[test]
    fn part1_real_input() {
        const INPUT_FILE: &str = "./input";
        let res = Part1(INPUT_FILE);
        const expected: i32 = 279;
        let res = (res.0.abs(), res.1.abs());
        let msg = format!("{} != {}", expected, (res.0 + res.1));
        assert!((res.0 + res.1) == expected, "{msg}");
    }

    #[test]
    fn test1_part2() {
        const INPUT_FILE: &str = "./input4";
        const expected: i32 = 4;
        let res = Part2(INPUT_FILE);
        let res = (res.0.abs(), res.1.abs());
        let msg = format!("{} != {}", expected, (res.0 + res.1));
        assert!((res.0 + res.1) == expected, "{msg}");
    }

    #[test]
    fn part2_real_input() {
        const INPUT_FILE: &str = "./input";
        const expected: i32 = 163;
        let res = Part2(INPUT_FILE);
        let res = (res.0.abs(), res.1.abs());
        let msg = format!("{} != {}", expected, (res.0 + res.1));
        assert!((res.0 + res.1) == expected, "{msg}");
    }
}
