use std::{iter::zip, vec};

fn check_if_zero(N:u32,D:&[u32],start:&[u32]) -> bool{
    for (d,r) in zip(D,start) {
        if ((N + r) % d) != 0 {return  false;}
    }
    true
}

fn Part1() -> u32 {
    const D:[u32;6] = [13,19,3,7,5,17];
    // const start:[u32;6] = [1,10,2,1,3,5];
    let mut start:[u32;6] = [1,10,2,1,3,5];

    // const D:[u32;2] = [5,2];
    // let mut start:[u32;2] = [4,1];

    for i in 0..start.len(){
        start[i] += (i+1) as u32;
    }

    for i in start.into_iter().min().unwrap()..1_000_000 {
        if check_if_zero(i as u32, &D, &start){
            return i;
        }
    }
    return 0;
}


fn Part2() -> u32 {
    const D:[u32;7] = [13,19,3,7,5,17,11];
    // const start:[u32;6] = [1,10,2,1,3,5];
    let mut start:[u32;7] = [1,10,2,1,3,5,0];

    // const D:[u32;2] = [5,2];
    // let mut start:[u32;2] = [4,1];

    for i in 0..start.len(){
        start[i] += (i+1) as u32;
    }

    for i in 10..1_000_000_000 {
        if check_if_zero(i as u32, &D, &start){
            return i;
        }
    }
    return 0;
}


fn main() {
    println!("Part 1 {}",Part1());
    println!("Part 2 {}",Part2());
}
