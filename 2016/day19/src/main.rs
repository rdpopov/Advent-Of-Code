// thsi is the knights problem https://en.wikipedia.org/wiki/Josephus_problem
fn Part1(elves:usize) -> usize {
    return ((elves & !(1 << elves.ilog2())) << 1 ) + 1 as usize;
}

fn Part2(elves_n:usize) -> usize {
    let  mut w = 1;
    for i in 1..elves_n {
        w = w % i + 1;
        if w > (i + 1)/2{
            w+=1;
        }
    }
    return w;
}

fn main() {
    println!("{}",Part1(3017957));
    println!("{}",Part2(3017957));
}
