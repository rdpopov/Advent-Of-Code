use std::{array, collections::HashSet, fs, str::FromStr, usize, os::unix::prelude};

fn clamp(n :i32 ,l:i32,u:i32) -> i32{
    if n <= l { return  l;}
    if n >= u { return  u;}
    n
}

fn parse_line(start:&mut (i32,i32),code:&str) -> i32 {
    // order doent really matter
    for i in code.chars(){
        match i {
            'U' => start = &mut (start.0 - 1,start.1),
            'D' => start = &mut (start.0 + 1,start.1),
            'L' => start = &mut (start.0 ,start.1 - 1),
            'R' => start = &mut (start.0 ,start.1 + 1),
            _=>()
        }
        start = &mut (clamp(start.0, 0, 2),clamp(start.1, 0, 2));
    }
    (start.0*3 + start.1 + 1)
}

fn Part1(INPUT_FILE: &str) -> i32 {
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let mut position = (0, 0);
    let mut result = 0;
    for line in contents.lines() {
        result = result * 10 + parse_line(&mut position, &line);
        println!("{:?} {}",position,result);
    }
    result
}


fn main() {
    const INPUT_FILE: &str = "./input_ex";
    println!("{:?}", Part1(INPUT_FILE))
}

mod test {
    use super::*;
// ULL
// RRDDD
// LURDL
// UUUUD
//
// 1985
//
// 1 2 3
// 4 5 6
// 7 8 9
//
    #[test]
    fn test1_parse_line_1() {
        const EXPECTED: i32 = 1;
        let mut start = (1,2); //5
        let res = parse_line(start, "ULL");
        let msg = format!("{} != {}", EXPECTED, res);
        assert!(res == EXPECTED, "{msg}");
    }

    #[test]
    fn test1_parse_line_2() {
        const EXPECTED: i32 = 9;
        let mut start = (0,0); //5
        let res = parse_line(start, "RRDDD");
        let msg = format!("{} != {}", EXPECTED, res);
        assert!(res == EXPECTED, "{msg}");
    }

    #[test]
    fn test1_parse_line_3() {
        const EXPECTED: i32 = 8;
        let mut start = (2,2); //5
        let res = parse_line(start, "LURDL");
        let msg = format!("{} != {}", EXPECTED, res);
        assert!(res == EXPECTED, "{msg}");
    }

    #[test]
    fn test1_parse_line_4() {
        const EXPECTED: i32 = 5;
        let mut start = (2,1); //5
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
