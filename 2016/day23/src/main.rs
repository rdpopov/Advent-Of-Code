use std::{collections::HashMap, fs};


fn instruction_transform(s:String) -> String{
    match s.as_str() {
        "inc" => "dec".to_string(),
        "dec" | "tgl" => "inc".to_string(),
        "jnz" => "cpy".to_string(),
        "cpy" => "jnz".to_string(),
        _ => "woops".to_string()
    }
}


fn Part1(INPUT_FILE: &str, reg_ov: Vec<(&str, i64)>) -> i64 {
    let mut inp: Vec<Vec<String>> = fs::read_to_string(INPUT_FILE)
        .expect("We dont have acess to tis file")
        .lines()
        .map(|x| x.split(" ").map(|f| f.to_owned()).collect::<Vec<String>>())
        .collect();
    let mut registers: HashMap<String, i64> = HashMap::from([("a".to_string(), 0), ("b".to_string(), 0), ("c".to_string(), 0), ("d".to_string(), 0)]);
    for (k, v) in reg_ov.into_iter() {
        registers.insert(k.to_string(), v);
    }
    let mut IP: i64 = 0; // inctruction pointer
    while 0 <= IP && IP < inp.len() as i64 {
        let instr: Vec<String> = inp[IP as usize].clone();
        match instr[0].as_str() {
            "cpy" => {
                let src: i64 = match registers.get(&instr[1]) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i64>().unwrap(),
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
            "jnz" => {
                let src: i64 = match registers.get(&instr[1]) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i64>().unwrap(),
                };
                if src != 0 {
                    IP += match registers.get(&instr[2]) {
                    Some(x) => x.to_owned(),
                    None => instr[2].parse::<i64>().unwrap(),
                };
                } else {
                    IP += 1;
                }
            }
            "tgl" => {
                let src: i64 = match registers.get(&instr[1]) {
                    Some(x) => x.to_owned(),
                    None => instr[1].parse::<i64>().unwrap(),
                };
                let idx_of_change = IP + src;

                if 0 <= idx_of_change && idx_of_change < inp.len() as i64{
                        let iinstruction = inp[idx_of_change as usize][0].to_owned();
                        inp[idx_of_change as usize][0] = instruction_transform(iinstruction);
                }

                IP+=1;
            }
            _ => panic!("No such instruction"),
        }
        // println!("Ip:{} {:?} {:?}",IP,instr,registers);
    }
    *registers.get(&"a".to_string()).unwrap()
}
fn main() {
    // println!("Part1 {}", Part1("input", vec![("a",7)]));
    println!("Part2 {}", Part1("input", vec![("a",12)]));
    // println!("Part1 {}", Part1("input_test", vec![]));
    // println!("Part2 {}", Part1("./input", vec![("c", 1)]));
}
mod test {
    #[test]
    fn test_example_input() {
        assert_eq!(3,crate::Part1("input_test", vec![("a",7)]));
    }
    #[test]
    fn test_part1() {
        assert_eq!(11500,crate::Part1("input", vec![("a",7)]));
    }
}
