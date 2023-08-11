use std::{collections::HashMap, fs, io::repeat};

fn Part1(INPUT_FILE: &str, reg_ov: Vec<(&str, i16)>) -> bool {
    let mut inp: Vec<Vec<String>> = fs::read_to_string(INPUT_FILE)
        .expect("We dont have acess to tis file")
        .lines()
        .map(|x| x.split(" ").map(|f| f.to_owned()).collect::<Vec<String>>())
        .collect();
    let pattern = [0,1];
    let mut patt_idx = 0;
    let mut hotspot: Vec<u64> = vec![0; inp.len()];
    let mut executed_instrucitons = 0;
    let mut registers: HashMap<String, i16> = HashMap::from([
        ("a".to_string(), 0),
        ("b".to_string(), 0),
        ("c".to_string(), 0),
        ("d".to_string(), 0),
    ]);
    for (k, v) in reg_ov.into_iter() {
        registers.insert(k.to_string(), v);
    }
    let mut IP: i64 = 0; // inctruction pointer
    while 0 <= IP && IP < inp.len() as i64 &&  executed_instrucitons<20 {
        hotspot[IP as usize] += 1;
        let instr: Vec<String> = inp[IP as usize].clone();
        match instr[0].as_str() {
            "cpy" => {
                let src: i16 = match registers.get(&instr[1]) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i16>().unwrap(),
                };
                if registers.contains_key(&instr[2]) {
                    registers.insert(instr[2].to_owned(), src);
                }
                IP += 1;
            }
            "dec" => {
                match registers.get_mut(&instr[1]) {
                    Some(x) => *x -= 1,
                    _ => panic!("No such key"),
                }
                IP += 1;
            }
            "inc" => {
                match registers.get_mut(&instr[1]) {
                    Some(x) => *x += 1,
                    _ => panic!("No such key"),
                }
                IP += 1;
            }
            "add" => {
                let src: i16 = match registers.get(&instr[1]) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i16>().unwrap(),
                };
                match registers.get_mut(&instr[2]) {
                    Some(x) => *x += src,
                    _ => panic!("No such key"),
                }
                IP += 1;
            }
            "jnz" => {
                let src: i16 = match registers.get(&instr[1]) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i16>().unwrap(),
                };
                if src != 0 {
                    IP += match registers.get(&instr[2]) {
                        Some(x) => x.to_owned() as i64,
                        None => instr[2].parse::<i16>().unwrap() as i64,
                    };
                } else {
                    IP += 1;
                }
            }
            "out" => {
                let src: i16 = match registers.get(&instr[1]) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i16>().unwrap(),
                };
                executed_instrucitons +=1;
                if pattern[patt_idx] == src {
                    patt_idx +=1;
                    patt_idx %=2;
                } else {
                    return false;
                }
                IP += 1;
            }
            "nop" => {
                IP += 1;
            }
            _ => panic!("No such instruction"),
        }
    }
    return true;
}


fn main() {
    // this is a bit shower thingy, prints the bits from the back repeatingly => 2555 + input should be
    // a number like 0b1010101010
    println!("result with 175 : {}",Part1("./input_mod", vec![("a", 175)]));
}