// thsi is the knights problem https://en.wikipedia.org/wiki/Josephus_problem
fn Part1(elves:usize) -> usize {
    return ((elves & !(1 << elves.ilog2())) << 1 ) + 1 as usize;
}

fn Part2(elves_n:usize) -> usize {
    let mut elves:Vec<usize> = vec![0;elves_n];
    let crnt_idx:usize = 0;
    for (i,el_n) in elves.iter_mut().enumerate(){
        *el_n = i;
    }
    while elves.len() > 0{
        let opposite_idx = crnt_idx + elves.len()/2;

            crnt_idx +=1;
    }
    0

}

fn main() {
    println!("{}",Part1(3017957));
}
