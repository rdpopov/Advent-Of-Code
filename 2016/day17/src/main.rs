($x:expr) => {
    1 <= $x.1 && $x.1 <= 3
};
}
macro_rules! can_move_left {
($y:expr) => {
    1 <= $y.0 && $y.0 <= 3
};
}

macro_rules! can_move_down {
($x:expr) => {
    0 <= $x.1 && $x.1 <= 2
};
}

macro_rules! can_move_right {
($y:expr) => {
    0 <= $y.0 && $y.0 <= 2
};
}
macro_rules! door_open {
($y:expr) => {
    'a' < $y && $y < 'g' 
}
}

#[derive(Debug,Clone,Default)]
struct State {
id: String,
xy: (u8,u8),
depth: u16 // maybe remove this
}

impl Hash for State {
fn hash<H: Hasher>(&self, state: &mut H) {
    self.depth.hash(state);
    self.xy.hash(state);
}
}


impl PartialEq for State {
fn eq(&self, other: &Self) -> bool {
    self.id == other.id
}
}
impl Eq for State {}

fn is_win(xy:(u8,u8))->bool{
xy.0 == 3 && xy.1 == 3
}

fn generate_transition(s:&State) -> Vec<State> {
let mut res:Vec<State> = vec![];
let hsh = format!("{:x}",md5::compute(&s.id));
let mut hsh_itr = hsh.chars().take(4).collect::<Vec<_>>(); 
if can_move_up!(s.xy)    && door_open!(hsh_itr[0]) { res.push(State{id:format!("{}U",&s.id),xy:(s.xy.0,s.xy.1-1),depth:s.depth+1}); }
if can_move_down!(s.xy)  && door_open!(hsh_itr[1]) { res.push(State{id:format!("{}D",&s.id),xy:(s.xy.0,s.xy.1+1),depth:s.depth+1}); }
if can_move_left!(s.xy)  && door_open!(hsh_itr[2]) { res.push(State{id:format!("{}L",&s.id),xy:(s.xy.0-1,s.xy.1),depth:s.depth+1}); }
if can_move_right!(s.xy) && door_open!(hsh_itr[3]) { res.push(State{id:format!("{}R",&s.id),xy:(s.xy.0+1,s.xy.1),depth:s.depth+1}); }
return res;
}


fn Part1(seed: &str) -> String {
let mut stk:PriorityQueue<State,i32>  = PriorityQueue::new();
stk.push(State { id: seed.to_string(), xy: (0,0), depth: 0 }, 0);
loop {
    if let Some(crnt) = stk.pop() {
        println!("{:?}",crnt);
        for i in generate_transition(&crnt.0) {
            if is_win(i.xy) {
                return i.id[seed.len()..].to_string();
            }
            stk.push(i.clone(), -(i.depth as i32));
        }
    } else {
        break;
    }
}
return "".to_string();
}

fn Part2(seed: &str) -> usize {
let mut stk:PriorityQueue<State,i32>  = PriorityQueue::new();
stk.push(State { id: seed.to_string(), xy: (0,0), depth: 0 }, 0);
let mut res:usize = 0;
loop {
    if let Some(crnt) = stk.pop() {
        for i in generate_transition(&crnt.0) {
            if is_win(i.xy) {
                println!("res: {} size: {}",res,stk.len());
                res = max(res, i.id.len() - seed.len());
            } else {
                stk.push(i.clone(), -(i.depth as i32));
            }
        }
    } else {
        break;
    }
}
return res;
}


fn main() {
println!("Part1 {}",Part1("qtetzkpl"));
println!("Part2 {}",Part2("qtetzkpl"));
}

// =======================================
// Part1 =================================
// =======================================

#[test]
fn test_ihgpwlah_part1() {
assert_eq!( Part1("ihgpwlah") ,"DDRRRD" );
}

#[test]
fn test_kglvqrro_part1() {
assert_eq!( Part1("kglvqrro") ,"DDUDRLRRUDRD" );
}

#[test]
fn test_ulqzkmiv_part1() {
assert_eq!( Part1("ulqzkmiv") ,"DRURDRUDDLLDLUURRDULRLDUUDDDRR" );
}

#[test]
fn test_qtetzkpl_part1() {
assert_eq!( Part1("qtetzkpl") ,"RRRLDRDUDD" );
}

// =======================================
// Part2 =================================
// =======================================

#[test]
fn test_ihgpwlah_part2() {
assert_eq!( Part2("ihgpwlah") ,370 );
}

#[test]
fn test_kglvqrro_part2() {
assert_eq!( Part2("kglvqrro") ,492 );
}

#[test]
fn test_ulqzkmiv_part2() {
assert_eq!( Part2("ulqzkmiv") ,830 );
}

#[test]
fn test_qtetzkpl_part2() {
assert_eq!( Part2("qtetzkpl") ,706 );
}