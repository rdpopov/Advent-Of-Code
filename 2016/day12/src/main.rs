use std::{collections::HashMap, fs};

fn Part1(INPUT_FILE: &str, reg_ov: Vec<(&str, i64)>) -> i64 {
    let inp: Vec<Vec<String>> = fs::read_to_string(INPUT_FILE)
        .expect("We dont have acess to tis file")
        .lines()
        .map(|x| x.split(" ").map(|f| f.to_owned()).collect::<Vec<String>>())
        .collect();
    let mut registers: HashMap<&str, i64> = HashMap::from([("a", 0), ("b", 0), ("c", 0), ("d", 0)]);
    for (k, v) in reg_ov.into_iter() {
        registers.insert(&k, v);
    }
    let mut IP: i64 = 0; // inctruction pointer
    while 0 <= IP && IP < inp.len() as i64 {
        let instr: &Vec<String> = &inp[IP as usize];
        match instr[0].as_str() {
            "cpy" => {
                let src: i64 = match registers.get(&instr[1].as_str()) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i64>().unwrap(),
                };
                registers.insert(&instr[2].as_str(), src);
                IP += 1;
            }
            "dec" => {
                match registers.get_mut(&instr[1].as_str()) {
                    Some(x) => *x -= 1,
                    _ => panic!("No such key"),
                }
                IP += 1;
            }
            "inc" => {
                match registers.get_mut(&instr[1].as_str()) {
                    Some(x) => *x += 1,
                    _ => panic!("No such key"),
                }
                IP += 1;
            }
            "jnz" => {
                let src: i64 = match registers.get(&instr[1].as_str()) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i64>().unwrap(),
                };
                if src != 0 {
                    IP += instr[2].parse::<i64>().unwrap();
                } else {
                    IP += 1;
                }
            }
            _ => panic!("No such instruction"),
        }
    }
    *registers.get(&"a").unwrap()
}
fn main() {
    println!("Part1 {}", Part1("./input", vec![]));
    println!("Part2 {}", Part1("./input", vec![("c", 1)]));
}
mod test {
    #[test]
    fn test_example_input() {
        let expected = 42;
        let result = crate::Part1("./input1", vec![]);
        assert!(expected == result, "expected {expected} != result {result}");
    }
}
