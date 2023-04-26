use std::vec;

enum Cell {
    Chip(char),
    Power(char),
}
fn Chip(chem: char, tbl: &Vec<char>) -> u32 {
    return 1 << tbl.iter().position(|&x| x == chem).unwrap();
}

fn Power(chem: char, tbl: &Vec<char>) -> u32 {
    return Chip(chem, tbl) << 16;
}

fn input(tbl: &Vec<char>) -> [u32; 4] {
    return [
        Power('S', tbl) | Chip('S', tbl) | Power('P', tbl) | Chip('P', tbl),
        Power('T', tbl) | Power('R', tbl) | Chip('R', tbl) | Power('C', tbl) | Chip('C', tbl),
        Chip('T', tbl),
        0,
    ];
}

fn is_legal_state(s: &[u32]) -> bool {
    let mut res = true;
    for i in s.iter() {
        res = res 
    }

}

fn input_ex(tbl: &Vec<char>) -> [u32; 4] {
    return [
        Chip('H',tbl) | Chip('L',tbl),
        Power('H',tbl),
        Power('L',tbl),
        0,
    ];
}
fn Part1() -> u32 {
    let mut inp = input(&vec!['S','P','T','R','C']);
    // let inp = input_ex(&vec!['H','L']);
    0
}

fn main() {
    println!("Part 1 {}", Part1());
}
