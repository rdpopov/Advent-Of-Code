fn has_n_consec(hsh:&String,n:i64) -> Option<Vec<char>> {
    let mut last: char = ' ';
    let mut sequence: i64 = 1;
    let mut found:Vec<char> = vec![] ;
    for (i,v) in hsh.chars().enumerate()  {
        if i != 0 {
            if v == last {
                sequence += 1
            } else {
                if sequence >= n {
                    found.push(last);
                }
                sequence = 1
            }
        }
        last = v;
    }
    if sequence >= n {
        found.push(last);
    }
    if found.len() > 0 {
        return  Some(found);
    }
    None
}
fn part1_compute(seed: &str, idx:usize) -> String {
    return  format!("{:?}",md5::compute(format!("{}{}", seed, idx))).to_string();
}

fn part2_compute(seed: &str, idx:usize) -> String {
    let mut dick_seed = format!("{:?}",md5::compute(format!("{}{}", seed, idx)));
    for i in 0..2016 {
        dick_seed = format!("{:?}",md5::compute(dick_seed));
    }
    dick_seed
}


fn solve(seed: &str,comput:&dyn Fn(&str, usize) -> String) -> usize {
    let mut found_hashes:u64 = 0;
    let mut idx:usize = 0;
    let mut queue:Vec<(char,usize,bool)> = Vec::new(); // vector of pars of hashes to watch out for
    let mut crnt_hash:String;
    let mut found_idx:Vec<(usize,usize)> = vec![];

    loop {
        crnt_hash = comput(seed, idx);
        if queue.len() > 0 && queue[0].1 + 1000 == idx {
            queue.remove(0);
        }
        if let Some(rep) = has_n_consec(&crnt_hash, 5) {
            for triple in queue.iter_mut() {
                if rep.contains(&triple.0) {
                    triple.2 = false;
                    found_hashes +=1;
                    found_idx.push((triple.1,idx));
               }
            }
            queue = queue.into_iter().filter(|&x| x.2).collect();
        }
        if let Some(rep) = has_n_consec(&crnt_hash, 3) {
            queue.push((rep[0],idx,true));
        }
        idx +=1;
        if idx % 1000 == 0{
            println!("@{idx}");
        }
        if found_hashes >= 70{ // need more than 64 sums to be calculated
            break;
        }
    }
    found_idx.sort();
    println!("@{:?}",found_idx);
    return found_idx[63].0;
}



fn main() {
    // println!("Part1 {}",solve("abc",&part1_compute));
    // println!("Part2 {}",solve("abc",&part2_compute));
    println!("Part1 {}",solve("ngcjuoqr",&part1_compute));
    println!("Part2 {}",solve("ngcjuoqr",&part2_compute));
}
