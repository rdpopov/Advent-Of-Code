fn gen_new_line(line: &str) -> String {
    const trap: u8 = 94;
    const free: u8 = 46;
    let line_itr = format!(".{line}.");
    line_itr
        .as_bytes()
        .windows(3)
        .map(|t| {
            char::from_u32(match t {
                [trap, trap, free] => trap, // left and center traps and right is free
                [free, trap, trap] => trap, // right and center traps and left is free
                [free, free, trap] => trap, // left and center free and right is trap
                [trap, free, free] => trap, // right and center free and left is trap
                _ => free,
            } as u32)
            .unwrap()
        })
        .collect::<String>()
}

fn Part1(seed: &str, n: i32) -> usize {
    let mut res = seed.chars().filter(|&c| c == '.').count();
    let mut line: String = seed.to_string();
    for i in 0..(n - 1) {
        line = gen_new_line(&line);
        res += line.clone().chars().filter(|&c| c == '.').count();
    }
    res
}

fn main() {
    const tst: &str = ".^^.^.^^^^";
    // println!("{}",gen_new_line(tst));
    println!("part1 {}", Part1( ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^" ,40));
    println!("part2 {}", Part1( ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^" ,400000));
}
