use std::fs;

fn main() {
    let mut floor:i32 = 0;
    let mut once:bool = true;

    for (pos,sym) in fs::read_to_string("./01/input").expect("No input").chars().enumerate() {
        if sym == '(' {
            floor+=1;
        } else if sym == ')' {
            floor-=1;
            if once && floor == -1 {
                println!("Santa found the dead orphans {}",pos+1);
                once = false;
            }
        }
    }
    println!("Santa is on floor {}",floor);
}
