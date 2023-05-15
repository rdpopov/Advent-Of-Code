use std::collections::HashSet;

const BOSS_NUMER: i32 = 1358;

fn isFree(x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x > 100 || y > 100 {
        return false;
    }
    let mut res = x * x + 3 * x + 2 * x * y + y + y * y + BOSS_NUMER;
    let mut bitcount = 0;
    while res != 0 {
        bitcount += res & 1;
        res >>= 1;
    }
    (bitcount & 1) == 0
}
const DIRS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn traverse_p1(
    x: i32,
    y: i32,
    target: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    paths: &mut Vec<u32>,
    path_len: u32,
) -> bool {
    if !isFree(x, y) {
        return false;
    }
    if visited.contains(&(x, y)) {
        return false;
    }
    if target == (x, y) {
        paths.push(path_len);
        return true;
    }
    visited.insert((x, y));
    for (dx, dy) in DIRS.iter() {
        traverse_p1(x + dx, y + dy, target, visited, paths, path_len + 1);
    }
    visited.remove(&(x, y));
    false
}

fn Part1(target: (i32, i32)) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut paths: Vec<u32> = Vec::new();
    traverse_p1(1, 1, target, &mut visited, &mut paths, 0);
    return paths.into_iter().min().unwrap();
}

fn traverse_p2(
    x: i32,
    y: i32,
    visited: &mut HashSet<(i32, i32)>,
    maze: &mut Vec<Vec<u32>>,
    path_len: u32,
) -> bool {
    if !isFree(x, y) {
        return false;
    }
    if maze[x as usize][y as usize] < path_len {
        return false;
    } else {
        maze[x as usize][y as usize] = path_len
    }
    // visited.insert((x,y));
    if path_len > 51 {
        return true;
    }
    for (dx, dy) in DIRS.iter() {
        traverse_p2(x + dx, y + dy, visited, maze, path_len + 1);
    }
    false
}

fn Part2() -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut maze: Vec<Vec<u32>> = vec![vec![std::u32::MAX; 100]; 100];
    traverse_p2(1, 1, &mut visited, &mut maze, 0);
    return maze
        .into_iter()
        .map(|x| x.into_iter().filter(|&x| x < 51).count() as u32)
        .sum();
}

fn main() {
    println!("part 1 {}",Part1((31,39)));
    println!("part 2 {}", Part2());
}
