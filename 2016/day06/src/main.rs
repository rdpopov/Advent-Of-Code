use std::fs;
use std::result;
use std::str;

fn Part1(INPUT_FILE: &str) -> String {
    let inp = fs::read_to_string(INPUT_FILE).expect("Could not read file");
    if let Some(len) = inp.find("\n") {
        if len != 0 {
            let mut freq_map: Vec<[i32; 255]> = vec![[0; 255]; len];
            for line in inp.lines() {
                for (i, c) in line.chars().enumerate() {
                    freq_map[i][c as usize] += 1;
                }
            }

            let result =  freq_map
                .into_iter()
                .map(|x| {
                    char::from_u32(
                        x.into_iter()
                            .enumerate()
                            .fold((0, 0), |acc, x| if acc.1 < x.1 { x } else { acc })
                            .0 as u32,
                    )
                    .expect("Caharcater should be in bound")
                })
                .collect::<String>();
            return result;
        } else {
            panic!("Wrong file format");
        }
    } else {
        panic!("No new line in the");
    }

    return "".to_string()
}

fn Part2(INPUT_FILE: &str) -> String {
    let inp = fs::read_to_string(INPUT_FILE).expect("Could not read file");
    if let Some(len) = inp.find("\n") {
        if len != 0 {
            let mut freq_map: Vec<[i32; 255]> = vec![[0; 255]; len];
            for line in inp.lines() {
                for (i, c) in line.chars().enumerate() {
                    freq_map[i][c as usize] += 1;
                }
            }

            let result =  freq_map
                .into_iter()
                .map(|x| {
                    char::from_u32(
                        x.into_iter()
                            .enumerate()
                            .fold((0, 255), |acc, x| if acc.1 > x.1 && x.1 != 0 { x } else { acc })
                            .0 as u32,
                    )
                    .expect("Caharcater should be in bound")
                })
                .collect::<String>();
            return result;
        } else {
            panic!("Wrong file format");
        }
    } else {
        panic!("No new line in the");
    }

    return "".to_string()
}



fn main() {
    const INPUT_FILE: &str = "./input";
    println!("Part 1 example {}",Part1(INPUT_FILE));
    println!("Part 2 example {}",Part2(INPUT_FILE));
}

#[test]
fn part1_example_input() {
    const INPUT_FILE: &str = "./input_ex";
    let result = Part1(INPUT_FILE);
    let expected = "easter";
    assert!(result == expected,"{result} != {expected}");
}

#[test]
fn part1_real_input() {
    const INPUT_FILE: &str = "./input";
    let result = Part1(INPUT_FILE);
    let expected = "liwvqppc";
    assert!(result == expected,"{result} != {expected}");
}


#[test]
fn part2_example_input() {
    const INPUT_FILE: &str = "./input_ex";
    let result = Part1(INPUT_FILE);
    let expected = "advent";
    assert!(result == expected,"{result} != {expected}");
}

#[test]
fn part2_real_input() {
    const INPUT_FILE: &str = "./input";
    let result = Part1(INPUT_FILE);
    let expected = "caqfbzlh";
    assert!(result == expected,"{result} != {expected}");
}

