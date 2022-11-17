use std::iter::{zip, OnceWith};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn vowel(s:&str) -> bool {
    let vowels = "aieou";
    let mut score:i32 = 0;
    for v in s.chars(){
        if vowels.contains(v) {
            score+=1;
        }
    }
    score > 2
}

fn double(s:&str) -> bool {
    if s.len() < 2 { return false };
    for i in 1..s.len() {
        if s.as_bytes()[i] == s.as_bytes()[i-1] {
            return true;
        }
    }
    false
}
fn not_in(s:&str) -> bool {
    let evil= [ "ab", "cd", "pq", "xy"];
    for i in evil {
        if s.contains(i) {
            return false;
        }
    }
    true
}

fn two_overlap(s:&str) -> bool {
    if s.len() < 4 { return false };
    for i in 1..s.len() {
        if s[i+1..].contains(&s[(i-1)..i+1]){
            return true;
        }
    }
    false
}

fn one_over(s:&str) -> bool {
    if s.len() < 3 { return false };
    for i in 2..s.len() {
        if s.as_bytes()[i] == s.as_bytes()[i-2] {
            return true;
        }
    }
    false
}


fn tst_str1(s:&str) -> bool{
    vowel(s) && double(s) && not_in(s)
}
fn tst_str2(s:&str) -> bool{
    two_overlap(s) && one_over(s)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn scenario_1() {
    println!("scenario_1");
    let mut res:i32 = 0;
    let arr = [
        "ugknbfddgicrmopn",
        "aaa",
        "jchzalrnumimnmhp",
        "haegwjzuvuyypxyu",
        "dvszwmarrgswjxmb"
    ];
    for line in arr {
        if tst_str1(&line){
            println!("{} is good",line);
        }
    }
    //====================================================
    if let Ok(lines) = read_lines("./src/input") {
        for line in lines {
            if tst_str1(&line.unwrap()){
                 res+=1;
            }
        }
    } else {
        println!("couldnt read file\n");
    }
    println!("good strings {}",res);
}

fn scenario_2 () {
    println!("scenario_2");
    let mut res:i32 = 0;
    let tst = [
        "qjhvhtzxzqqjkmpb"
            ,"xxyxx"
            ,"uurcxstgmygtbstg"
            ,"ieodomkazucvgmuy"];

    for line in tst {
        if tst_str2(&line){
            println!("{} is good",line);
        }
    }
    // =================================================
    if let Ok(lines) = read_lines("./src/input") {
        for line in lines {
            if tst_str2(&line.unwrap()){
                 res+=1;
            }
        }
    }
    println!("good strings {}",res);
}


fn main() {
    scenario_1();
    scenario_2();
}
