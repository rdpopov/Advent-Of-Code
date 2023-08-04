use std::{collections::{HashSet, HashMap},hash::Hash, hash::Hasher, fmt::{Display, format, Debug}, clone};


#[derive(Copy, Eq ,Clone)]
union State {
    repr:[u8;8],
    packed:u64
}

impl Default for State {
    fn default() -> Self {
        Self{packed: 0}
    }
}

fn new(repr:u64) -> State{
    State{packed:repr}
}

fn init_part1(Elv:u8, chips:&[u8],gens : &[u8]) -> State {
    let mut res = State::default();
    unsafe {
        res.repr[Elv as usize *2] = 128;
        for (i,f) in chips.iter().enumerate() {
            res.repr[*f as usize * 2 + 1] |= (1 << i ) as u8;
        }
        for (i,f) in gens.iter().enumerate() {
            res.repr[*f as usize * 2]  |= (1 << i ) as u8;
        }
    }
    res
}



impl PartialEq for State { 
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            return self.packed == other.packed
        }
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res:String = "".to_string();
        unsafe {
            let items = self.repr[1] | self.repr[3] | self.repr[5] | self.repr[7];
            for i in (0..4).rev() {
                res = format!( "{res}F{i} {}", if (self.repr[i as usize * 2] & 128) == 128 { "E "} else {". "});
                let mut idx = 0;
                while 1 << idx < items{
                    res = format!( "{res}{} {} ",   if (self.repr[i as usize * 2] & (1<< idx)) != 0 { format!("G{idx}")} else {". ".to_string()},
                    if (self.repr[i as usize * 2 + 1] & (1<< idx)) != 0 { format!("M{idx}")} else {". ".to_string()});
                    idx += 1;
                }
                res = format!("{res}\n");
            }
            write!(f, "{res}")
        }
    }
}
impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            write!(f, "{}",self.packed)
        }
    }

}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.packed.hash(state);
        }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if self.packed == other.packed {
                return std::cmp::Ordering::Equal
            } else if  self.packed <= other.packed {
                return std::cmp::Ordering::Less
            } else {
                return std::cmp::Ordering::Greater
            }
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe {
            if self.packed == other.packed {
                return Some(std::cmp::Ordering::Equal)
            } else if  self.packed <= other.packed {
                return Some(std::cmp::Ordering::Greater)
            } else {
                return Some(std::cmp::Ordering::Less)
            }
        }
    }
}
// sym diff bits -> sdb
fn sdb(a:u8,b:u8) -> u8 {
    a ^ b & a
}
fn is_win(s:State) -> bool {
    unsafe {
        let items = s.repr[1] | s.repr[3] | s.repr[5] | s.repr[7];
        return items == s.repr[6] & 127 && items == s.repr[7] & 127;
    }
}

fn is_legal(s:State) -> bool {
    for i in 0..4 {
        unsafe {
            let l = s.repr[i*2] & 127; // clean the elevator bit just in case
            let r = s.repr[i*2 + 1];
            if sdb(l,r) > 0 && sdb(r,l) > 0 {
                return false;
            }
        }
    }
    true
}

fn diff_moves(s: State) -> HashSet<State> {
    let mut res = HashSet::new();
    let mut elev_pos = 4;
    let mut items =0;
    unsafe {
        items = s.repr[1] | s.repr[3] | s.repr[5] | s.repr[7];
    }
    for i in 0..4{
        unsafe {
            if s.repr[i*2] & 128 != 0 {
                elev_pos = i;
            }
        }
    }
    assert!(elev_pos <= 3);
    unsafe {
        let mut chps:u8 = s.repr[elev_pos*2 + 1] & 127;
        let mut gens:u8 = s.repr[elev_pos*2] & 127;
        let mut mask = 1;
        // one chip
        while mask < items {
            let ones = mask.count_ones();
            if ones == 1 || ones == 2 {
                if elev_pos > 0 {
                    let mut ns = s.clone();
                    // remove elevator
                    ns.repr[2*elev_pos] ^= 128;
                    ns.repr[2*(elev_pos - 1)] |=  128;
                    // add component
                    if chps & mask != 0 {
                        let mut only_chip = ns.clone();
                        only_chip.repr[2*elev_pos +1] ^= mask;
                        only_chip.repr[2*(elev_pos - 1) +1] |= mask;
                        if is_legal(only_chip) { res.insert(only_chip);}
                    }
                    if gens & mask != 0 {
                        let mut only_gens = ns.clone();
                        only_gens.repr[2*elev_pos] ^= mask;
                        only_gens.repr[2*(elev_pos - 1)] |= mask;
                        if is_legal(only_gens) { res.insert(only_gens);}
                    }
                    if mask & gens !=0 && chps & mask != 0 && mask.count_ones() == 1 {
                        ns.repr[2*elev_pos +1] ^= mask;
                        ns.repr[2*(elev_pos - 1) +1] |= mask;
                        ns.repr[2*elev_pos] ^= mask;
                        ns.repr[2*(elev_pos - 1)] |= mask;
                        if is_legal(ns) { res.insert(ns);}
                    }
                }
                if elev_pos < 3 {
                    let mut ns = s.clone();
                    // remove elevator
                    ns.repr[2*elev_pos] ^= 128;
                    ns.repr[2*(elev_pos+1)] |= 128;
                    // add component
                    if chps & mask != 0 {
                        let mut only_chip = ns.clone();
                        only_chip.repr[2*elev_pos + 1] ^= mask;
                        only_chip.repr[2*(elev_pos + 1) +1] |= mask;
                        if is_legal(only_chip) { res.insert(only_chip);}
                    }
                    if gens & mask != 0 {
                        let mut only_gens = ns.clone();
                        only_gens.repr[2*elev_pos] ^= mask;
                        only_gens.repr[2*(elev_pos + 1)] |= mask;
                        if is_legal(only_gens) { res.insert(only_gens);}
                    }
                    if mask & gens !=0 && chps & mask != 0 && mask.count_ones() == 1 {
                        ns.repr[2*elev_pos +1] ^= mask;
                        ns.repr[2*(elev_pos + 1) +1] |= mask;
                        ns.repr[2*elev_pos] ^= mask;
                        ns.repr[2*(elev_pos + 1)] |= mask;
                        if is_legal(ns) { res.insert(ns);}
                    }
                }
            }
            mask += 1;
        }
    }
    return res;
}

fn btw(v: u8, arg_1: u8, arg_2: u8, d_elv: u8) -> bool {
    arg_1 <= v + d_elv && v + d_elv <= arg_2
} 

fn generate_new_states(s:State,visited: &HashMap<State,u8>) -> HashSet<State> {
    let next_states = diff_moves(s);
    return HashSet::from_iter(next_states.into_iter().filter(|x| !visited.contains_key(x)));
}

fn Part1(s:State,depth:u8) -> u8 {
    let mut visited:HashMap<State,u8> = HashMap::new();
    let mut crnt_states :HashSet<State> = HashSet::new();

    crnt_states.insert(s);

    visited.insert(s,0);
    for dpth in 0..depth {
        let mut next_states :HashSet<State> = HashSet::new();
        for i in crnt_states.drain() {
            let nxt = generate_new_states(i, &visited);
            next_states.extend(nxt);
        }
        for i in &next_states {
            // println!("{}",*i);
            if is_win(*i) {
                // println!("{}\n{}",*i,dpth+1);
                return dpth+1;
            }
            visited.insert(*i,dpth+1);
        }
        println!("depth:{dpth} next: {}", next_states.len());
        crnt_states.extend(next_states);
    }
    0
}

fn Part1_pruning(s:State,depth:u8) -> u8 {
    let mut visited:HashMap<State,u8> = HashMap::new();
    let mut crnt_states :HashSet<State> = HashSet::new();
    let prune_size  = 100000;

    crnt_states.insert(s);

    visited.insert(s,0);
    for dpth in 0..depth {
        let mut next_states :HashSet<State> = HashSet::new();
        let all_crnt_states = crnt_states.len();
        for i in crnt_states.drain() {
            let nxt = generate_new_states(i, &visited);

            next_states.extend(nxt);
        }
        for i in &next_states {
            if is_win(*i) {
                println!("depth:{dpth} win");
                return dpth+1;
            }
            visited.insert(*i,dpth+1);
        }
        let mut pruned:Vec<State> =  next_states.clone().into_iter().collect();
        let mut pruned_set = HashSet::new();
        pruned.sort();
        // for i in &pruned {
        //     println!("{i}\n{:?}\n\n",i);
        // }
        for x in pruned.into_iter().take(prune_size) {
            pruned_set.insert(x);
        }
        println!("depth:{dpth} next: {}", next_states.len());
        crnt_states.extend(pruned_set);
    }
    0
}

fn main() {
    //  |S |P |T |R |C |
    // G|0 |0 |1 |1 |1 |
    // M|0 |0 |2 |1 |1 |
    let start:State = init_part1(0,&[0 ,0 ,2 ,1 ,1] , &[0 ,0 ,1 ,1 ,1]);
    println!("Part 1 {}", Part1_pruning(start,200));

    // let start:State = init_part1(0, &[0,0], &[1,2]);
    // println!("Part 1 {}", Part1_pruning(start,11));
}


#[test]
fn two_generators_input() {
    let start:State = init_part1(0, &[0,0], &[1,2]);
    assert_eq!(11,Part1(start,11))
}

#[test]
fn two_generators_prune_function_input() {
    let start:State = init_part1(0, &[0,0], &[1,2]);
    assert_eq!(11,Part1_pruning(start,11))
}



