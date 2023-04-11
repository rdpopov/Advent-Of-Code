use std::fs;

enum State {
    ConsumeLiteral,
    ReadMacro,
    ReadLetters,
}

fn parse_line_part1(l: &str) -> String {
    let mut state: State = State::ConsumeLiteral;
    let mut Macro: String = "".to_string();
    let mut ToConsume: usize = 0;
    let mut TimesToRepeat: usize = 0;
    let mut result: String = "".to_string();
    for c in l.chars() {
        match state {
            State::ConsumeLiteral => match c {
                '(' => state = State::ReadMacro,
                _ => result.push(c),
            },
            State::ReadMacro => match c {
                ')' => {
                    let m_args = Macro
                        .split('x')
                        .map(|n| n.parse::<usize>().expect(""))
                        .collect::<Vec<_>>();
                    ToConsume = m_args[0];
                    TimesToRepeat = m_args[1];
                    Macro.clear();
                    state = State::ReadLetters;
                }
                _ => Macro.push(c),
            },
            State::ReadLetters => {
                Macro.push(c);
                if ToConsume == Macro.len() {
                    for i in 0..TimesToRepeat {
                        result.extend(Macro.chars());
                    }
                    Macro.clear();
                    state = State::ConsumeLiteral;
                }
            }
        }
    }
    result
}
fn Part1(INPUT_FILE: &str) -> usize {
    let imp = fs::read_to_string(INPUT_FILE).expect("Cant rea file {INPUT_FILE}");
    let mut sum: usize = 0;

    for i in imp.lines() {
        sum += parse_line_part1(i).len();
    }
    sum
}
fn Part2(INPUT_FILE: &str) -> u32 {
    0
}
fn main() {
    const INPUT_FILE: &str = "input";
    println!("Part 1 output = {}", Part1(INPUT_FILE));
    println!("Part 2 output = {}", Part2(INPUT_FILE));
}

#[test]
fn test_input_1_part1() {
    let generated = parse_line_part1("ADVENT");
    let expected = "ADVENT".to_string();
    assert!(expected == generated, "result {generated} != {expected}");
}

#[test]
fn test_input_2_part1() {
    let generated = parse_line_part1("A(1x5)BC");
    let expected = "ABBBBBC".to_string();
    assert!(expected == generated, "result {generated} != {expected}");
}

#[test]
fn test_input_3_part1() {
    let generated = parse_line_part1("(3x3)XYZ");
    let expected = "XYZXYZXYZ".to_string();
    assert!(expected == generated, "result {generated} != {expected}");
}
