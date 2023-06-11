use std::{vec, collections::HashSet, cmp::min};

fn Chip(chem: char, tbl: &Vec<char>) -> u32 {
    return 1 << tbl.iter().position(|&x| x == chem).unwrap();
}


fn Power(chem: char, tbl: &Vec<char>) -> u32 {
    return Chip(chem, tbl) << 16;
}
fn PrintState(s:([u32;4],usize,u32), tbl: &Vec<char>) {
    PrintStateImpl(s.0,s.1,tbl);
}
fn PrintStateImpl(floors:[u32;4],elevator:usize, tbl: &Vec<char>) {
        println!("------------------");
    for i in (0..4).rev() {
        let (u,l) = upper_and_lower(floors[i]);
        print!("floor {i}: ");
        for chem in 0..tbl.len() {
            if (u & (1 << chem)) > 0 {
                print!("{}G ",tbl[chem])
            }
            if (l & (1 << chem)) > 0 {
                print!("{}M ",tbl[chem])
            }
        }
        if elevator == i {
            print!("E");
        }
        print!("\n");
    }
}

fn upper_and_lower(layer:u32) -> (u32,u32) {
    ((layer & 0xFFFF0000) >> 16, layer & 0xFFFF)
}
fn is_win_state(st:[u32;4],tbl: &Vec<char>) -> bool {
    let (u,l) = upper_and_lower(st[3]);
    let full_mask = 1 << (tbl.len() + 1) - 1;
    if u == l && u == full_mask {
        return true;
    }
    false
}

fn input(tbl: &Vec<char>) -> [u32; 4] {
    return [
        Power('S', tbl) | Chip('S', tbl) | Power('P', tbl) | Chip('P', tbl),
        Power('T', tbl) | Power('R', tbl) | Chip('R', tbl) | Power('C', tbl) | Chip('C', tbl),
        Chip('T', tbl),
        0,
    ];
}

fn is_legal_state(s: &[u32]) -> bool {
    let mut res = true;
    for i in s.iter() {
        let (u,l) = upper_and_lower(*i);
        let xr = u ^ l;
        if (u & l) > 0 {
            return true;
        }
    }
    false
}
fn generate_state_for_floor(st:[u32;4],crnt_floor:usize,next_floor:usize,steps:u32) -> Vec<([u32;4],usize,u32)> {
    let mut res = vec![];
    let (u,l) = upper_and_lower(st[crnt_floor]);
    let mut idx:u32 = 1;
    // move one generator
    while idx < u {
        if u & idx > 0 {
            let mut tmp_state = st.to_owned();
            tmp_state[next_floor] |= (idx << 16);
            tmp_state[crnt_floor] ^= (idx << 16);
            if is_legal_state(&tmp_state) {
                res.push((tmp_state,next_floor,steps +1));
            }
        }
        idx <<= 1;
    }
    idx = 1;
    // move one chip
    while idx < l {
        if l & idx > 0 {
            let mut tmp_state = st.to_owned();
            tmp_state[next_floor] |= (idx);
            tmp_state[crnt_floor] ^= (idx);
            if is_legal_state(&tmp_state) {
                res.push((tmp_state,next_floor,steps +1));
            }
        }
        idx <<= 1;
    }

    // move two chips
    let mut idx_0:u32 = 1;
    let mut idx_1:u32 = 1;
    while idx_0 < u {
        idx_1 = 1;
        while idx_1 < u {
            if u & idx_0 > 0 && u & idx_1 > 0  && idx_1 > idx_0{
                let mut tmp_state = st.to_owned();
                tmp_state[next_floor] |= (idx_0 | idx_1) << 16;
                tmp_state[crnt_floor] ^= (idx_0 | idx_1) << 16;
                if is_legal_state(&tmp_state) {
                    res.push((tmp_state,next_floor,steps +1));
                }
            }
            idx_1 <<= 1;
        }
        idx_0 <<= 1;
    }

    // move two chips
    let mut idx_0:u32 = 1;
    let mut idx_1:u32 = 1;
    while idx_0 < l {
        idx_1 = 1;
        while idx_1 < l {
            if l & idx_0 > 0 && l & idx_1 > 0  && idx_1 > idx_0{
                let mut tmp_state = st.to_owned();
                tmp_state[next_floor] |= (idx_0 | idx_1);
                tmp_state[crnt_floor] ^= (idx_0 | idx_1);
                if is_legal_state(&tmp_state) {
                    res.push((tmp_state,next_floor,steps +1));
                }
            }
            idx_1 <<= 1;
        }
        idx_0 <<= 1;
    }
    res.sort();
    res
}
fn generate_states(st:([u32;4],usize,u32)) -> Vec<([u32;4],usize,u32)> {
    let floor = st.1;
    let mut res = vec![];
    if floor > 0 {
        res.append(&mut generate_state_for_floor(st.0,floor , floor-1, st.2));
    }
    if floor < 3 {
        res.append(&mut generate_state_for_floor(st.0,floor , floor+1, st.2));
    }
    res
}

fn input_ex(tbl: &Vec<char>) -> [u32; 4] {
    return [
        Chip('H',tbl) | Chip('L',tbl),
        Power('H',tbl),
        Power('L',tbl),
        0,
    ];
}

fn Part1() -> u32 {
    // let mut inp = input(&vec!['S','P','T','R','C']);
    let tbl = &vec!['H','L'];
    let mut inp = input_ex(&tbl);
    let mut states:Vec<Vec<([u32;4],usize,u32)>> = vec![vec![(inp,0,0)]];
    let mut visited:HashSet<[u32;4]>=HashSet::new();
    // PrintState(states[0][0],tbl);
    let mut idx = 0;
    let mut min_step = u32::MAX;
    let mut backtrack = false;

    while states.len() > 0 {
        let last = states.len();
        if last == 0 {break;}
        if let Some(crnt_state) = states[last-1].pop() {
            visited.insert(crnt_state.0);
            PrintState(crnt_state, tbl);
            if is_win_state(crnt_state.0, tbl) {
                min_step = min(min_step,crnt_state.2);
            }
            let new_states:Vec<_> = generate_states(crnt_state).into_iter().filter(|x| !visited.contains(&x.0)).collect();
            for i in new_states.iter(){
                PrintState(*i, tbl);
            }
            println!("depth {last} new states: {}",new_states.len());
            states.push(new_states);
        } else {
            states.pop();
        };
        // break
    }
    min_step
}

fn main() {
    // println!("Part 1 {}", Part1());
    println!("Part 1 {}", Part1());

}
