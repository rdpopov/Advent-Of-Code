use std::{collections::{HashSet, HashMap},hash::Hash, hash::Hasher, fmt::Display};

#[derive(Copy, Eq ,Clone)]
union State {
    repr:[i8;8],
    packed:i64
}

const ELEMS:Vec<char> = vec!['H','L'];
// const ELEMS:Vec<char> = vec!['S','P','T','R','C'];

impl Default for State {
    fn default() -> Self {
        Self{packed: 0}
    }
}

fn new(repr:i64) -> State{
    State{packed:repr}
}

fn init_part1(Elv:i8, chips:&[i8],gens : &[i8]) -> State {
    let mut res = State::default();
    unsafe {
        res.repr[Elv as usize *2] = 128;
    }
    for i in chips{
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
        // TODO:  rework this to be more in line with the elements thing
        write!(f, "{res}")
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
        self.packed.hash(state);
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe {
            if self.packed == other.packed {
                return Some(std::cmp::Ordering::Equal)
            } else if  self.packed <= other.packed {
                return Some(std::cmp::Ordering::Less)
            } else {
                return Some(std::cmp::Ordering::Greater)
            }
        }
    }
}

fn is_legal(s:State) -> bool {
    // TODO: check these properly
    false
}

fn diff_move(s: State, chips:&[i8],gens : &[i8]) -> State {
    return State::default();
}

fn btw(v: i8, arg_1: i8, arg_2: i8, d_elv: i8) -> bool {
    arg_1 <= v + d_elv && v + d_elv <= arg_2
} 

fn generate_new_states(s:State,visited: &HashMap<State,i8>) -> HashSet<State> {
    let next_states = vec![
        // TODO: Generate new states 
    ];

    return HashSet::from_iter(next_states.into_iter().filter(|x| !visited.contains_key(x)));
}
fn Part1(s:State,depth:i8) -> i8 {
    let mut visited:HashMap<State,i8> = HashMap::new();
    let mut crnt_states :HashSet<State> = HashSet::new();

    crnt_states.insert(s);

    for dpth in 0..depth {
        let mut next_states :HashSet<State> = HashSet::new();
        for i in crnt_states.drain() {
            let nxt = generate_new_states(i, &visited);
            next_states.extend(nxt);
        }
        for i in &next_states {
            visited.insert(*i,dpth);
        }
        crnt_states.extend(next_states);
    }
    0
}

fn main() {
    // let start:State = init_part1(0, 0, 0,1, 2);
    let k
    println!("{}");
    //

    // Part1(start,15);
}
