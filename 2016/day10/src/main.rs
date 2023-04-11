use std::{collections::HashMap, fs, process::Output, ptr::hash};

fn main() {
    const INPUT_FILE: &str = "input_ex";
    Part1(INPUT_FILE);
}

#[derive(Debug, PartialEq)]
enum Drain {
    Bot(usize),
    Output(usize),
}

impl Default for Drain {
    fn default() -> Self {
        Self::Output(0)
    }
}

#[derive(Debug, Default)]
struct Robot {
    input: Vec<u32>,
    low_idx: Drain,
    high_idx: Drain,
}

fn build_rules(inp: &str) -> (HashMap<usize, Robot>, HashMap<usize, Vec<u32>>) {
    let mut result: HashMap<usize, Robot> = HashMap::new();
    let mut outputs: HashMap<usize, Vec<u32>> = HashMap::new();
    for i in inp.lines() {
        let mut tok = i.split(' ');
        if tok.nth(0).unwrap() == "bot" {
            let bot_id: usize = tok.nth(0).unwrap().parse::<usize>().unwrap();

            let low_out: Drain = match tok.nth(3) {
                Some("bot") => Drain::Bot(tok.nth(0).unwrap().parse::<usize>().unwrap()),
                Some("output") => {
                    let val = tok.nth(0).unwrap().parse::<usize>().unwrap();
                    outputs.insert(val, vec![]);
                    Drain::Output(val)
                },
                _ => panic!("how did we get here"),
            };

            let high_out: Drain = match tok.nth(3) {
                Some("bot") => Drain::Bot(tok.nth(0).unwrap().parse::<usize>().unwrap()),
                Some("output") => {
                    let val = tok.nth(0).unwrap().parse::<usize>().unwrap();
                    outputs.insert(val, vec![]);
                    Drain::Output(val)
                }
                _ => panic!("how did we get here"),
            };
            result.insert(
                bot_id,
                Robot {
                    input: vec![],
                    low_idx: low_out,
                    high_idx: high_out,
                },
            );
        }
    }
    // println!("{:#?}", result);
    // println!("{:#?}", outputs);
    return (result,outputs);
}

fn Part1(input_file: &str) {
    let inp = fs::read_to_string(input_file).expect("Sohuld be able to read file");
    let (automata,outputs) = build_rules(&inp);
}
