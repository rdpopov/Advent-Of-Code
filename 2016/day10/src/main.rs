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
                }
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
    println!("{:#?}", result);
    println!("{:#?}", outputs);
    return (result, outputs);
}
enum Roundresult {
    Played,
    NoStart,
}
fn playRound(
    automata: &mut HashMap<usize, Robot>,
    outputs: &mut HashMap<usize, Vec<u32>>,
) -> Result<Roundresult,usize> {
    let mut played: bool = false;
    let mut current_bot_key: Vec<usize> = vec![];
    let mut res = Ok(Roundresult::NoStart);

    for (k, v) in automata.into_iter() {
        if v.input.len() == 2 {
            current_bot_key.push(*k);
            res = Ok(Roundresult::Played);
            break;
        }
    }

    loop {
        if current_bot_key.len() > 0 {
            let bot = current_bot_key.remove(0);

            let robot:&Robot = automata.get(&bot).unwrap().;
            // this is iffy V
            let mut temp_array = robot.input.to_owned().into_iter().take(2).collect::<Vec<u32>>(); 

            temp_array.sort();
            let min = temp_array[0];
            let max = temp_array[1];

            match robot.low_idx {
                Drain::Bot(x) => automata.get_mut(&x).unwrap().input.push(min),
                Drain::Output(x) => outputs.get_mut(&x).unwrap().push(min),
            }

            match robot.high_idx {
                Drain::Bot(x) => automata.get_mut(&x).unwrap().input.push(max),
                Drain::Output(x) => outputs.get_mut(&x).unwrap().push(max),
            }

        } else {
            break;
        }
    }

    res
}

fn Part1(input_file: &str) {
    let inp = fs::read_to_string(input_file).expect("Sohuld be able to read file");
    let (automata, outputs) = build_rules(&inp);
}
