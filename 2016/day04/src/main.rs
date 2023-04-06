use std::{fs, usize, cmp::Ordering};
fn is_real_room(l: &str) -> bool {
    let mut assoc_arr: [u32; 255] = [0; 255];
    let mut toks: Vec<&str> = l.split("-").collect();
    let hash_and_code: Vec<&str> = toks.remove(toks.len() - 1).split("[").collect();
    let code = from_str(hash_and_code[0]);
    let hash_bit = toks[1].strip_suffix(']').unwrap_or(toks[1]);

    for i in toks {
        for c in i.chars() {
            assoc_arr[c as usize] += 1;
        }
    }
    let mut to_sort: Vec<_> = assoc_arr
        .into_iter()
        .enumerate()
        .filter(|&(_, n)| n > 0)
        .map(|(x, n)| ( char::from_u32(x as u32).unwrap(),n))
        // .map(|(x, n)| (n, char::from_u32(x as u32).unwrap()))
        .collect();

    to_sort.sort_by(|&(n,x),&(k,y)| if n.eq(&k) {x.partial_cmp(&y).unwrap()} else {n.partial_cmp(&k).unwrap()} );
    // let actual_hash: String = to_sort
    //     .into_iter()
    //     .collect();

    println!("{:?}", &to_sort);
    false
}

fn Part1(INPUT_FILE: &str) -> i32 {
    let contents = fs::read_to_string(INPUT_FILE).expect("Should have been able to read the file");
    let mut position = 5;
    let mut result = 0;
    for line in contents.lines() {
        is_real_room(line);
    }
    result
}
// fn Part2(input_file: &str) -> i32 {
//     let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
//     let mut position = 5;
//     let mut result = 0;
//     for line in contents.lines() {
//         let new_pos = parse_line_part2(position, &line);
//         result = result * 16 + new_pos;
//         position = new_pos;
//     }
//     result
// }

fn from_str(i: &str) -> u32 {
    let mut res: u32 = 0;
    for j in i.chars() {
        match j {
            '0'..='9' => res = res * 10 + (j.to_digit(10).unwrap()),
            _ => (),
        }
    }
    res
}

fn main() {
    Part1("./input_ex");
}
