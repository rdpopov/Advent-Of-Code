// thsi is the knights problem https://en.wikipedia.org/wiki/Josephus_problem
fn Part1(elves:usize) -> usize {
    return ((elves & !(1 << elves.ilog2())) << 1 ) + 1 as usize;
}

fn main() {
    println!("{}",Part1(3017957));
}
