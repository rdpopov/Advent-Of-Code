use std::{
    clone,
    collections::{HashMap, HashSet},
    fmt::{format, Debug, Display},
    hash::Hash,
    hash::Hasher,
};

#[derive(Copy, Eq, Clone)]
union State {
    repr: [u8; 8],
    packed: u64,
}

impl Default for State {
    fn default() -> Self {
        Self { packed: 0 }
    }
}

fn new(repr: u64) -> State {
    State { packed: repr }
}

macro_rules! gens_floor {
    ($x:expr) => {
        ($x * 2) as usize
    };
}
macro_rules! chips_floor {
    ($x:expr) => {
        ($x * 2 + 1) as usize
    };
}

fn init_part1(Elv: u8, chips: &[u8], gens: &[u8]) -> State {
    let mut res = State::default();
    unsafe {
        res.repr[gens_floor!(Elv)] = 128;
        for (i, f) in chips.iter().enumerate() {
            res.repr[chips_floor!(*f)] |= (1 << i) as u8;
        }
        for (i, f) in gens.iter().enumerate() {
            res.repr[gens_floor!(*f)] |= (1 << i) as u8;
        }
    }
    res
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return self.packed == other.packed }
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = "".to_string();
        unsafe {
            let items = self.repr[chips_floor!(0)]
                | self.repr[chips_floor!(1)]
                | self.repr[chips_floor!(2)]
                | self.repr[chips_floor!(3)];
            for i in (0..4).rev() {
                res = format!(
                    "{res}F{i} {}",
                    if (self.repr[gens_floor!(i)] & 128) == 128 {
                        "E "
                    } else {
                        ". "
                    }
                );
                let mut idx = 0;
                while 1 << idx < items {
                    res = format!(
                        "{res}{} {} ",
                        if (self.repr[gens_floor!(i)] & (1 << idx)) != 0 {
                            format!("G{idx}")
                        } else {
                            ". ".to_string()
                        },
                        if (self.repr[chips_floor!(i)] & (1 << idx)) != 0 {
                            format!("M{idx}")
                        } else {
                            ". ".to_string()
                        }
                    );
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
        unsafe { write!(f, "{}", self.packed) }
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
                return std::cmp::Ordering::Equal;
            } else if self.packed <= other.packed {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe {
            if self.packed == other.packed {
                return Some(std::cmp::Ordering::Equal);
            } else if self.packed <= other.packed {
                return Some(std::cmp::Ordering::Greater);
            } else {
                return Some(std::cmp::Ordering::Less);
            }
        }
    }
}
// sym diff bits -> sdb
fn sdb(a: u8, b: u8) -> u8 {
    a ^ b & a
}
// if all elemnts are on one floor and there were no elemnts higher, moves until winning condition can
// be estimated with a function (2 * (2*items - 1) - 1) * (floors - crnt_floor)
// this wont work as i keep the ones that are the biggest as a heuristic.
fn left_moves(s: State) -> i32 {
    unsafe {
        let items = (s.repr[chips_floor!(0)]
                   | s.repr[chips_floor!(1)]
                   | s.repr[chips_floor!(2)]
                   | s.repr[chips_floor!(3)])
        .count_ones() as i32;
        let floors_to_move = ((s.repr[0] & 128).count_ones()) * 3
            + ((s.repr[2] & 128).count_ones()) * 2
            + ((s.repr[4] & 128).count_ones());
        return (2 * (2 * items - 1) - 1) * floors_to_move as i32;
    }
}

fn is_all_on_one_line(s: State, start: State) -> bool {
    unsafe {
        let highest_not_changed = highest_element(s) == highest_element(start);
        let items = s.repr[chips_floor!(0)]
                   | s.repr[chips_floor!(1)]
                   | s.repr[chips_floor!(2)]
                   | s.repr[chips_floor!(3)];
        return (items == s.repr[gens_floor!(1)] & 127 && items == s.repr[chips_floor!(1)] & 127)
            || (items == s.repr[gens_floor!(2)] & 127 && items == s.repr[chips_floor!(2)] & 127)
            || (items == s.repr[gens_floor!(3)] & 127 && items == s.repr[chips_floor!(3)] & 127)
                && highest_not_changed;
    }
}
fn is_win(s: State, _: State) -> bool {
    unsafe {
        let items = s.repr[chips_floor!(0)]
                  | s.repr[chips_floor!(1)]
                  | s.repr[chips_floor!(2)]
                  | s.repr[chips_floor!(3)];
        return items == s.repr[gens_floor!(3)] & 127 && items == s.repr[chips_floor!(3)] & 127;
    }
}

fn is_legal(s: State) -> bool {
    for i in 0..4 {
        unsafe {
            let l = s.repr[gens_floor!(i)] & 127; // clean the elevator bit just in case
            let r = s.repr[chips_floor!(i)];
            if sdb(l, r) > 0 && sdb(r, l) > 0 {
                return false;
            }
        }
    }
    true
}

fn diff_moves(s: State) -> HashSet<State> {
    let mut res = HashSet::new();
    let mut elev_pos = 4;
    let mut items = 0;
    unsafe {
        let items = s.repr[chips_floor!(0)]
                  | s.repr[chips_floor!(1)]
                  | s.repr[chips_floor!(2)]
                  | s.repr[chips_floor!(3)];
        for i in 0..4 {
            if s.repr[gens_floor!(i)] & 128 != 0 {
                elev_pos = i;
            }
        }
        assert!(elev_pos <= 3);
        let mut chps: u8 = s.repr[chips_floor!(elev_pos)] & 127;
        let mut gens: u8 = s.repr[gens_floor!(elev_pos)] & 127;
        let mut mask: u8 = 1;
        // one chip
        while mask < items {
            let ones = mask.count_ones();
            if ones == 1 || ones == 2 {
                if elev_pos > 0 {
                    let mut ns = new(s.packed);
                    // remove elevator
                    ns.repr[gens_floor!(elev_pos)] &= !128;
                    ns.repr[gens_floor!(elev_pos - 1)] |= 128;
                    // add components
                    if chps & mask == mask {
                        let mut only_chip: State = new(ns.packed);
                        only_chip.repr[chips_floor!(elev_pos)] &= !mask;
                        only_chip.repr[chips_floor!(elev_pos - 1)] |= mask;
                        if is_legal(only_chip) {
                            res.insert(only_chip);
                        };
                    }
                    if gens & mask == mask {
                        let mut only_gens = new(ns.packed);
                        only_gens.repr[gens_floor!(elev_pos)] &= !mask;
                        only_gens.repr[gens_floor!(elev_pos - 1)] |= mask;
                        if is_legal(only_gens) {
                            res.insert(only_gens);
                        }
                    }
                    if mask & gens == mask && chps & mask == mask && ones == 1 {
                        ns.repr[chips_floor!(elev_pos)] &= !mask;
                        ns.repr[chips_floor!(elev_pos - 1)] |= mask;
                        ns.repr[gens_floor!(elev_pos)] &= !mask;
                        ns.repr[gens_floor!(elev_pos - 1)] |= mask;
                        if is_legal(ns) {
                            res.insert(ns);
                        }
                    }
                }
                if elev_pos < 3 {
                    let mut ns = new(s.packed);
                    // remove elevator
                    ns.repr[gens_floor!(elev_pos)] &= !128;
                    ns.repr[gens_floor!(elev_pos + 1)] |= 128;
                    // add components
                    if chps & mask == mask {
                        let mut only_chip = new(ns.packed);
                        only_chip.repr[chips_floor!(elev_pos)] = chps & !mask;
                        only_chip.repr[chips_floor!(elev_pos + 1)] |= mask;
                        if is_legal(only_chip) {
                            res.insert(only_chip);
                        }
                    }
                    if gens & mask == mask {
                        let mut only_gens = new(ns.packed);
                        only_gens.repr[gens_floor!(elev_pos)] &= !mask;
                        only_gens.repr[gens_floor!(elev_pos + 1)] |= mask;
                        if is_legal(only_gens) {
                            res.insert(only_gens);
                        }
                    }
                    if mask & gens == mask && chps & mask == mask && ones == 1 {
                        ns.repr[chips_floor!(elev_pos)] &= !mask;
                        ns.repr[chips_floor!(elev_pos + 1)] |= mask;
                        ns.repr[gens_floor!(elev_pos)] &= !mask;
                        ns.repr[gens_floor!(elev_pos + 1)] |= mask;
                        if is_legal(ns) {
                            res.insert(ns);
                        }
                    }
                }
            }
            mask += 1;
        }
    }
    return res;
}

fn highest_element(s: State) -> u8 {
    unsafe {
        return ((s.repr[gens_floor!(1)] & 128) >> 7) * 1
            + ((s.repr[gens_floor!(2)] & 128) >> 7) * 2;
    }
}

fn btw(v: u8, arg_1: u8, arg_2: u8, d_elv: u8) -> bool {
    arg_1 <= v + d_elv && v + d_elv <= arg_2
}

fn generate_new_states(s: State, visited: &HashMap<State, u8>) -> HashSet<State> {
    let next_states = diff_moves(s);
    return HashSet::from_iter(next_states.into_iter().filter(|x| !visited.contains_key(x)));
}

type WinCond = fn(State, State) -> bool;

fn Part1(s: State, cond: WinCond, depth: u8) -> u8 {
    let mut visited: HashMap<State, u8> = HashMap::new();
    let mut crnt_states: HashSet<State> = HashSet::new();
    let prune_size = 550; // can be lower but want to have a buffer
    crnt_states.insert(s);
    visited.insert(s, 0);
    for dpth in 0..depth {
        let mut next_states: HashSet<State> = HashSet::new();
        for i in crnt_states.drain() {
            let nxt = generate_new_states(i, &visited);
            next_states.extend(nxt);
        }
        for i in &next_states {
            if cond(*i, *i) {
                println!("won at depth {} left moves {}", dpth + 1, left_moves(*i));
                println!("{i}");
                return dpth + 1 + left_moves(*i) as u8;
            }
            visited.insert(*i, dpth + 1);
        }
        let mut pruned: Vec<State> = next_states.clone().into_iter().collect();
        let mut pruned_set = HashSet::new();
        pruned.sort();
        for x in pruned.into_iter().take(prune_size) {
            pruned_set.insert(x);
        }
        crnt_states.extend(pruned_set);
    }
    0
}

fn main() {
    //  |S |P |T |R |C |
    // G|0 |0 |1 |1 |1 |
    // M|0 |0 |2 |1 |1 |
    //
    let start: State = init_part1(0, &[0, 1, 1, 2, 2], &[0, 0, 0, 2, 2]);
    // let start: State = init_part1(0, &[0, 0, 2, 1, 1], &[0, 0, 1, 1, 1]);
    println!("Part 1 {}", Part1(start, is_win, 200));

    let start: State = init_part1(0, &[0, 1, 1, 2, 2, 0 ,0], &[0, 0, 0, 2, 2, 0 ,0]);
    // let start: State = init_part1(0, &[0, 0, 2, 1, 1, 0, 0], &[0, 0, 1, 1, 1, 0, 0]);
    println!("Part 2 {}", Part1(start, is_win, 200));
}

#[test]
fn left_moves_2_items_floor_3() {
    let start: State = init_part1(2, &[2, 2], &[2, 2]);
    assert_eq!(5, left_moves(start));
}

#[test]
fn all_one_line_pass() {
    let start: State = init_part1(0, &[1, 1], &[1, 1]);
    assert_eq!(true, is_all_on_one_line(start, start));
}

#[test]
fn all_one_line_fail() {
    let start: State = init_part1(0, &[2, 1], &[1, 1]);
    assert_eq!(false, is_all_on_one_line(start, start));
}

#[test]
fn two_generators_input() {
    let start: State = init_part1(0, &[0, 0], &[1, 2]);
    assert_eq!(11, Part1(start, is_win, 11))
}

#[test]
fn test_input_part1_is_win() {
    let start: State = init_part1(0, &[0, 0, 2, 1, 1], &[0, 0, 1, 1, 1]);
    assert_eq!(37, Part1(start, is_win, 200))
}

#[test]
fn test_input_part2_is_win() {
    let start: State = init_part1(0, &[0, 0, 2, 1, 1, 0, 0], &[0, 0, 1, 1, 1, 0, 0]);
    assert_eq!(61, Part1(start, is_win, 200))
}

#[test]
fn test_input_part1_ex2_is_win() {
    let start: State = init_part1(0, &[0, 1, 1, 2, 2], &[0, 0, 0, 2, 2]);
    assert_eq!(31, Part1(start, is_win, 200))
}

#[test]
fn test_input_part2_ex2_is_win() {
    let start: State = init_part1(0, &[0, 1, 1, 2, 2, 0 ,0], &[0, 0, 0, 2, 2, 0 ,0]);
    assert_eq!(55, Part1(start, is_win, 200))
}
