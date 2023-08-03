use core::fmt;
use std::{cmp::min, collections::{HashSet, HashMap}, path::Display, vec, mem::swap};

#[derive(Debug, PartialEq, Default, Clone)]
struct State {
    chps: Vec<i32>,
    gens: Vec<i32>,
    tbl: Box<Vec<char>>,
    elevator: u32,
    iterations: u32,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: String = "".to_string();
        for i in (0..4).rev() {
            for j in 0..self.tbl.len() {
                if self.chps[j] == i {
                    res = format!("{res} {}M ", self.tbl[j]);
                } else {
                    res = format!("{res} .. ");
                }
            }
            for j in 0..self.tbl.len() {
                if self.gens[j] == i {
                    res = format!("{res} {}G ", self.tbl[j]);
                } else {
                    res = format!("{res} .. ");
                }
            }
            res = format!("{res}\n");
        }
        write!(f, "{res}")
    }
}
fn between (i: i32 , l:i32, u:i32) -> bool {
    l <= i && i <= u
}

fn input_ex() -> State {
    State {
        chps: vec![0, 2],
        gens: vec![0, 2],
        tbl: Box::new(vec!['H', 'L']),
        elevator: 0,
        iterations: 0,
    }
}
fn is_legal_state(s: &State) -> bool {
    for i in 0..4 {
        let gens: HashSet<_> = s.gens.iter().filter(|x| **x == i).enumerate().collect();
        let chip: HashSet<_> = s.chps.iter().filter(|x| **x == i).enumerate().collect();
        let diff = gens.difference(&chip).count();
        let diff1 = chip.difference(&gens).count();
        if diff > 0 && diff1 > 0 {
            return false;
        }
    }
    true
}

fn is_win(s: &State) -> bool {
    if s.chps.iter().min() == Some(&3) && s.gens.iter().min() == Some(&3) {
        return true;
    }
    false
}
fn heuristic(s: &State) -> i32 {
    let mut res: i32 = 0;
    for c in s.chps.iter() {
        res += 3 - c;
    }
    for g in s.gens.iter() {
        res += 3 - g;
    }
    res }
fn generate_next_floor(s: &State, dir: i32) -> Vec<State> {
    let mut res = vec![];
    let gen_moves:Vec<_> = s
        .gens
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&x| x.1 == s.elevator as i32)
        .map(|x| (x.0, x.1 + dir)).collect();
    let chp_moves:Vec<_> = s
        .chps
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&x| x.1 == s.elevator as i32)
        .map(|x| (x.0, x.1 + dir)).collect();

    // println!("gen_moves {:?} ",gen_moves);
    // println!("chp_moves {:?} ",chp_moves);

    let new_floor = (s.elevator as i32 + dir) as u32;
    // move one or 2 chips
    for i in 0..chp_moves.len() {
        for j in i..chp_moves.len() {
            if j == i {
                let mut c_new = s.chps.clone();
                if between(chp_moves[j].1,0,3) {
                    c_new[chp_moves[j].0  as usize] += dir;
                    let s = State { chps: c_new, gens: s.gens.clone(), tbl: s.tbl.clone() , elevator: new_floor, iterations: s.iterations+1 };
                    if is_legal_state(&s) {
                        res.push(s);
                    }
                }
            } else {
                let mut c_new = s.chps.clone();
                if between(chp_moves[j].1 ,0,3)
                && between(chp_moves[i].1 ,0,3)
                {
                    c_new[chp_moves[j].0 as usize] += dir;
                    c_new[chp_moves[i].0 as usize] += dir;
                    let s = State { chps: c_new, gens: s.gens.clone(), tbl: s.tbl.clone() , elevator: new_floor, iterations: s.iterations+1 };
                    if is_legal_state(&s) {
                        res.push(s);
                    }
                }
            }
        }
    }

    for i in 0..gen_moves.len() {
        for j in i..gen_moves.len() {
            if j == i {
                let mut g_new = s.gens.clone();
                if between(gen_moves[j].1 as i32 ,0,3) {
                    g_new[gen_moves[j].0  as usize] += dir;
                    let s = State { chps: s.chps.clone(), gens: g_new , tbl: s.tbl.clone() , elevator: new_floor, iterations: s.iterations+1 };
                    if is_legal_state(&s) {
                        res.push(s);
                    }
                }
            } else {
                let mut g_new = s.gens.clone();
                if between(gen_moves[j].1 as i32 ,0,3)
                && between(gen_moves[i].1 as i32 ,0,3)
                {
                    g_new[gen_moves[j].0 as usize] += dir;
                    g_new[gen_moves[i].0 as usize] += dir;
                    let s = State { chps: s.chps.clone(), gens: g_new , tbl: s.tbl.clone() , elevator: new_floor, iterations: s.iterations+1 };
                    if is_legal_state(&s) {
                        res.push(s);
                    }
                }
            }
        }
    }

    for i in 0..gen_moves.len() {
        for j in i..chp_moves.len() {
            if gen_moves[i] == chp_moves[j] {
                let mut g_new = s.gens.clone();
                let mut c_new = s.chps.clone();
                if between(gen_moves[i].1 as i32 ,0,3)
                && between(chp_moves[j].1 as i32 ,0,3)
                {
                    g_new[gen_moves[i].0 as usize] += dir;
                    c_new[chp_moves[j].0 as usize] += dir;
                    let s = State { chps: c_new, gens: g_new , tbl: s.tbl.clone() , elevator: new_floor, iterations: s.iterations+1 };
                    if is_legal_state(&s) {
                        res.push(s);
                    }
                }
            }
        }
    }

    res
}

fn generate_moves(s: &State) -> Vec<State> {
    let mut res = vec![];
    if s.elevator < 3 {
        res.append(&mut generate_next_floor(&s, 1));
    }
    if s.elevator > 0 {
        res.append(&mut generate_next_floor(&s, -1));
    }
    res
}

fn generate_next_n_moves(s: &State, n:i32) -> Vec<State> {
    let mut res = vec![];
    let mut alt = vec![s.clone()];
    // let pruned: HashMap<(Vec<i32>,Vec<i32>),Vec<State>> = HashMap::new();
    let mut done : HashSet<(Vec<i32>,Vec<i32>)> = HashSet::new();
    let mut solutions:Vec<State> =vec![];
    let mut minh = (heuristic(&s),s.clone());
    for i in 0..n { 
            for i in alt.iter() {
                for j in generate_moves(&i) {
                    if is_win(&j) {
                        solutions.push(j.clone());
                    }
                    // if done.insert((j.chps.clone(),j.gens.clone())) {
                    let h_res = heuristic(&j);
                    if h_res < minh.0 {
                        // minh = (h_res,j.clone());
                        res.push(j)
                    }
                    // }
                }
            }
            swap(&mut res, &mut alt);
            res.clear();
    }
    if !solutions.is_empty() {
        return solutions;
    }
    return alt;

}

fn Part1() -> u32 {
    let init = input_ex();
    // println!("{}", is_legal_state(&init));
    let mut current_cand : Vec<State>;

    println!("{:?} - initial", &init);
    for i in generate_next_n_moves(&init,1) {
        println! ("{}\n===========================", &i);
    }

    // print!("{:?}",generate_next_n_moves(&init,11)[0]);
    // generate_moves(&init);
    0
}

fn main() {
    println!("Part 1 {}", Part1());
}
