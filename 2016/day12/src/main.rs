use std::{collections::HashMap, fs};

fn Part1(INPUT_FILE: &str) -> i32 {
    let inp: Vec<Vec<String>> = fs::read_to_string(INPUT_FILE)
        .expect("We dont have acess to tis file")
        .lines()
        .map(|x| x.split(" ").map(|f| f.to_owned()).collect::<Vec<String>>())
        .collect();
    let mut registers: HashMap<&str, &mut i32> = HashMap::new();

    let IP: i32 = 0; // inctruction pointer
    while 0 <= IP && IP < inp.len() as i32 {
        let instr: &Vec<String> = &inp[IP as usize];
        match instr[0].as_str() {
            "cpy" => match registers.get(&instr[1].as_str()) {
                Some(x) => match registers.get(&instr[2].as_str()) {
                    Some(y) => *y = *x,
                    _ => panic!("No such key"),
                },
                _ => panic!("No such key"),
            },
            "dec" => match registers.get_mut(&instr[1].as_str()) {
                Some(x) => *x -= 1,
                _ => panic!("No such key"),
            },
            "inc" => match registers.get_mut(&instr[1].as_str()) {
                Some(x) => *x += 1,
                _ => panic!("No such key"),
            },
            "jnz" => (),
            _ => panic!("No such instruction"),
        }
    }

    print!("{:#?}", inp);
    0
}
fn main() {
    Part1("./input");
}
