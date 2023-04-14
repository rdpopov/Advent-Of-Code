use std::{collections::HashMap, fs, process::Output, ptr::hash};

#[derive(Debug, PartialEq, Clone)]
enum Drain {
    Bot(usize),
    Output(usize),
}

impl Default for Drain {
    fn default() -> Self {
        Self::Output(0)
    }
}

#[derive(Debug, Default, Clone)]
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
    // println!("{:#?}", result);
    // println!("{:#?}", outputs);
    return (result, outputs);
}
enum Roundresult {
    Played,
    NoStart,
}
// target is a tuple of indexes, needed for the solution,
// -> which node compares 17 and 63?
// -> also should be sorted
fn playRound(
    automata: &mut HashMap<usize, Robot>,
    outputs: &mut HashMap<usize, Vec<u32>>,
    target: (u32, u32),
) -> Result<usize, usize> {
    let mut current_bot_key: Vec<usize> = vec![];

    for (k, v) in automata.into_iter() {
        if v.input.len() >= 2 {
            current_bot_key.push(*k);
            break;
        }
    }

    loop {
        if current_bot_key.len() > 0 {
            let bot = current_bot_key.remove(0);

            //get uinput
            let mut robot: Robot = automata.get(&bot).unwrap().clone();
            // this is iffy V
            if robot.input.len() >= 2 {
                robot.input.resize(2, 0);

                robot.input.sort();
                let min = robot.input[0];
                let max = robot.input[1];
                if min == target.0 && max == target.1 {
                    // We are just going to use the error propagation functionality of rust
                    return Err(bot);
                }

                // update children
                match robot.low_idx {
                    Drain::Bot(x) => {
                        // println!("from {bot} to {x} send {min}");
                        automata.get_mut(&x).unwrap().input.push(min);
                        current_bot_key.push(x);
                    }
                    Drain::Output(x) => outputs.get_mut(&x).unwrap().push(min),
                }

                match robot.high_idx {
                    Drain::Bot(x) => {
                        // println!("from {bot} to {x} send {max}");
                        automata.get_mut(&x).unwrap().input.push(max);
                        current_bot_key.push(x);
                    }
                    Drain::Output(x) => outputs.get_mut(&x).unwrap().push(max),
                }
                // remove first 2
                automata.get_mut(&bot).unwrap().input.remove(0);
                automata.get_mut(&bot).unwrap().input.remove(0);
            } else {
                continue;
            }
        } else {
            break;
        }
    }
    Ok(0)
}
fn add_to_input(automata: &mut HashMap<usize, Robot>, value: u32, bot_key: usize) {
    automata
        .get_mut(&bot_key)
        .expect("How did we miss this bot?")
        .input
        .push(value);
}

fn Part1(input_file: &str, target: (u32, u32)) -> Result<usize, usize> {
    let inp = fs::read_to_string(input_file).expect("Sohuld be able to read file");
    let (mut automata, mut outputs) = build_rules(&inp);
    for l in inp.lines() {
        let mut tok = l.split(' ');
        match tok.nth(0).unwrap() {
            "bot" => (),
             "value"=> add_to_input(
                &mut automata,
                tok.nth(0).unwrap().parse::<u32>().unwrap(),
                tok.nth(3).unwrap().parse::<usize>().unwrap()
            ),
            _ => (),
        }
        playRound(&mut automata, &mut outputs, target)?;
    }

    return Ok(0);
}

fn Part2(input_file: &str, target: (u32, u32)) -> Result<usize, usize> {
    let inp = fs::read_to_string(input_file).expect("Sohuld be able to read file");
    let (mut automata, mut outputs) = build_rules(&inp);
    for l in inp.lines() {
        let mut tok = l.split(' ');
        match tok.nth(0).unwrap() {
            "bot" => (),
            "value"=> add_to_input(
                &mut automata,
                tok.nth(0).unwrap().parse::<u32>().unwrap(),
                tok.nth(3).unwrap().parse::<usize>().unwrap()
            ),
            _ => (),
        }
        playRound(&mut automata, &mut outputs, target)?;
    }
    println!("{:#?}",&outputs.get(&0).unwrap()[0] * &outputs.get(&1).unwrap()[0] * &outputs.get(&2).unwrap()[0] );

    return Ok(0);
}
fn main() {
    const INPUT_FILE: &str = "input";
    println!("{:?}",Part1(INPUT_FILE, (17, 61)));
    println!("{:?}",Part2(INPUT_FILE, (0, 0)));
}
