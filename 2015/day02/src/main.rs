use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn area(arg: &str) -> (i32,i32) {
    let mut sizes:Vec<i32> = arg.split('x').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    sizes.sort();
    (2*sizes[2]*sizes[1] + 3*sizes[1]*sizes[0] + 2*sizes[0]*sizes[2],
     sizes[0]*2 + sizes[1]*2 + sizes[0]*sizes[1]*sizes[2])

}

fn test(arg: &str) {
    println!("res for {:?} is {:?}",arg,area(arg));
}

fn read_lines<P> (fname: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    test("2x3x4");
    test("1x1x10");
    let mut sum:i32 = 0;
    let mut rib:i32 = 0;
    for i in read_lines("./input").unwrap(){
        let dims = area(&i.unwrap());
        sum += dims.0;
        rib += dims.1;
    }
    println!("total szie {:?}",(sum,rib));
}
