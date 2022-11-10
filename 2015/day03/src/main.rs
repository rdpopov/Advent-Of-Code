use std::{collections::HashSet, fs::read_to_string};


fn loc(crnt:(i32,i32) ,arg: char) -> (i32,i32) {
    match arg {
        '^' => {(crnt.0,crnt.1+1)}
        '>' => {(crnt.0+1,crnt.1)}
        '<' => {(crnt.0-1,crnt.1)}
        'v' => {(crnt.0,crnt.1-1)}
        _ => {crnt}
    }
}

fn path(movments:&str) -> i32 {
    let mut houses = HashSet::<(i32,i32)>::new();
    let mut pos = (0,0);
    houses.insert(pos);
    for i in movments.chars(){
        pos = loc(pos,i);
        houses.insert(pos);

    }
    houses.len() as i32
}

fn path2(movments:&str) -> i32 {
    let mut houses = HashSet::<(i32,i32)>::new();
    let mut pos_arr = [(0,0), (0,0)];
    houses.insert(pos_arr[0]);
    for (s,i) in movments.chars().enumerate(){
        pos_arr[s%2] = loc(pos_arr[s%2],i);
        houses.insert(pos_arr[s%2]);
    }
    houses.len() as i32
}
fn test(arg: &str) {
    println!("res for {:?} is {:?}",arg,path(arg));
}
fn test2(arg: &str) {
    println!("res for {:?} is {:?}",arg,path2(arg));
}

fn main() {
    test(">");
    test("^>v<");
    test("^v^v^v^v^v");
    test2(">");
    test2("^>v<");
    test2("^v^v^v^v^v");
    println!("houses: {}",path(&read_to_string("./src/input").unwrap()));
    println!("houses next year: {}",path2(&read_to_string("./src/input").unwrap()));
}
