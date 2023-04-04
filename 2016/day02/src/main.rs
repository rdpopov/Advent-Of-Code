use std::{array, collections::HashSet, fs, os::unix::prelude, str::FromStr, usize};

fn clamp(n: i32, l: i32, u: i32) -> i32 {
    if n <= l {
        return l;
    }
    if n >= u {
        return u;
    }
    n
}

fn parse_line(n: i32, code: &str) -> i32 {
    // order doent really matter
    let mut start = ((n - 1) / 3, (n - 1) % 3);
    for i in code.chars() {
        match i {
            'U' => start = (clamp(start.0 - 1,0,2), start.1),
            'D' => start = (clamp(start.0 + 1,0,2), start.1),
            'L' => start = (start.0, clamp(start.1 - 1,0,2)),
            'R' => start = (start.1, clamp(start.1 + 1,0,2)),
            _ => (),
        }
    }
    println!("{:?}",&start);
    (start.0 * 3 + start.1 + 1)
}

fn Part1(INPUT_FILE: &str) -> i32 {
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let mut position = 5;
    let mut result = 0;
    for line in contents.lines() {
        let new_pos = parse_line(position, &line);
        result = result * 10 + new_pos;
        position = new_pos;
        println!("{:?} {}", &position, &result);
    }
    result
}

fn main() {
    const INPUT_FILE: &str = "./input_ex";
    println!("{:?}", Part1(INPUT_FILE))
}

mod test {
    use super::*;

    #[test]
    fn test1_parse_line_1() {
        const EXPECTED: i32 = 1;
        let start = 5;
        let res = parse_line(start, "ULL");
        let msg = format!("{} != {}", EXPECTED, res);
        assert!(res == EXPECTED, "{msg}");
    }

    #[test]
    fn test1_parse_line_2() {
        const EXPECTED: i32 = 9;
        let start = 1;
        let res = parse_line(start, "RRDDD");
        let msg = format!("{} != {}", EXPECTED, res);
        assert!(res == EXPECTED, "{msg}");
    }

    #[test]
    fn test1_parse_line_3() {
        const EXPECTED: i32 = 8;
        let start = 9;
        let res = parse_line(start, "LURDL");
        let msg = format!("{} != {}", EXPECTED, res);
        assert!(res == EXPECTED, "{msg}");
    }

    #[test]
    fn test1_parse_line_4() {
        const EXPECTED: i32 = 5;
        let start = 8;
        let res = parse_line(start, "UUUUD");
        let msg = format!("{} != {}", EXPECTED, res);
        assert!(res == EXPECTED, "{msg}");
    }

    #[test]
    fn test1_part1_eample() {
        const INPUT_FILE: &str = "./input_ex";
        const expected: i32 = 1985;
        let res = Part1(INPUT_FILE);
        let msg = format!("{} != {}", expected, res);
        assert!(res == expected, "{msg}");
    }

    #[test]
    fn test1_part1_real_input() {
        const INPUT_FILE: &str = "./input";
        const expected: i32 = 0;
        let res = Part1(INPUT_FILE);
        let msg = format!("{} != {}", expected, res);
        assert!(res == expected, "{msg}");
    }
    // ===========================
    // #[test]
    // fn test2_part1() {
    //     const INPUT_FILE: &str = "./input2";
    //     const expected: i32 = 2;
    //     let res = Part1(INPUT_FILE);
    //     let msg = format!("{} != {}", expected, res);
    //     assert!(res == expected, "{msg}");
    // }

    // #[test]
    // fn test3_part1() {
    //     const INPUT_FILE: &str = "./input3";
    //     const expected: i32 = 12;
    //     let res = Part1(INPUT_FILE);
    //     let msg = format!("{} != {}", expected, res);
    //     assert!(res == expected, "{msg}");
    // }

    // #[test]
    // fn part1_real_input() {
    //     const INPUT_FILE: &str = "./input";
    //     let res = Part1(INPUT_FILE);
    //     const expected: i32 = 279;
    //     let msg = format!("{} != {}", expected, res);
    //     assert!(res == expected, "{msg}");
    // }

    // #[test]
    // #[ignore]
    // fn test1_part2() {
    //     const INPUT_FILE: &str = "./input4";
    //     const expected: i32 = 4;
    //     let res = Part1(INPUT_FILE);
    //     let msg = format!("{} != {}", expected, res);
    //     assert!(res == expected, "{msg}");
    // }

    // #[test]
    // fn part2_real_input() {
    //     const INPUT_FILE: &str = "./input";
    //     const expected: i32 = 163;
    //     let res = Part1(INPUT_FILE);
    //     let msg = format!("{} != {}", expected, res);
    //     assert!(res == expected, "{msg}");
    // }
}
