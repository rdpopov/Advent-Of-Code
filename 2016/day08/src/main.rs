use std::{cmp::min, fmt::Display, fs};

const SegLine: usize = 50;
const SegCols: usize = 6;
type SegmentDisplay = [[u8; SegLine]; SegCols];

fn print_display(dsp: &SegmentDisplay) {
    for i in dsp {
        println!(
            "{}",
            i.into_iter()
                .map(|&x| if x == 1 { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

fn print_letters(dsp: &SegmentDisplay) {
    for i in dsp {
        for j in 0..(SegLine / 5) {
            print!(
                "{}",
                &i[(j * 5)..((j + 1) * 5)]
                    .into_iter()
                    .map(|&x| if x == 1 { '#' } else { ' ' })
                    .collect::<String>()
            );
        }
        println!("")
    }
}

fn turn_on(dsp: &mut SegmentDisplay, x_size: usize, y_size: usize) {
    for y in 0..min(y_size, SegCols) {
        for x in 0..min(x_size, SegLine) {
            dsp[y][x] = 1;
        }
    }
}

fn shift_col_n(dsp: &mut SegmentDisplay, col: usize, _shif_n: usize) {
    // let mut temp_col:[u8;SegCols] = [0;SegCols];
    let shift_n = _shif_n % SegCols;
    let mut slc: Vec<u8> = vec![0; shift_n];
    for i in 0..shift_n {
        slc[i] = dsp[i + (SegCols - shift_n)][col];
    }
    for i in (shift_n..SegCols).rev() {
        dsp[i][col] = dsp[i - shift_n][col];
    }
    for i in 0..shift_n {
        dsp[i][col] = slc[i];
    }
}

fn shift_row_n(dsp: &mut SegmentDisplay, row: usize, _shif_n: usize) {
    let shift_n = _shif_n % SegLine;
    let mut slc: Vec<u8> = vec![0; shift_n];
    for i in 0..shift_n {
        slc[i] = dsp[row][i + (SegLine - shift_n)];
    }
    for i in (shift_n..SegLine).rev() {
        dsp[row][i] = dsp[row][i - shift_n];
    }
    for i in 0..shift_n {
        dsp[row][i] = slc[i];
    }
}

fn Part1(INPUT_FILE: &str) -> u32 {
    let inp = fs::read_to_string(INPUT_FILE).expect("File does not exist {INPUT_FILE}");
    let mut dsp: SegmentDisplay = [[0; SegLine]; SegCols];

    for i in inp.lines() {
        let tok = i
            .split(&['=', ' ', 'x'])
            .filter(|&x| x.len() > 0)
            .collect::<Vec<_>>();
        match tok[0] {
            "rect" => turn_on(
                &mut dsp,
                tok[1].parse::<usize>().unwrap(),
                tok[2].parse::<usize>().unwrap(),
            ),
            "rotate" => match tok[1] {
                "column" => shift_col_n(
                    &mut dsp,
                    tok[2].parse::<usize>().unwrap(),
                    tok[4].parse::<usize>().unwrap(),
                ),
                "row" => shift_row_n(
                    &mut dsp,
                    tok[3].parse::<usize>().unwrap(),
                    tok[5].parse::<usize>().unwrap(),
                ),
                _ => panic!("Command unknown"),
            },
            _ => panic!("Command unknown"),
        }
    }
    return dsp
        .into_iter()
        .map(|x| x.into_iter().map(|x| x as u32).sum::<u32>())
        .sum();
}

fn Part2(INPUT_FILE: &str) -> u32 {
    let inp = fs::read_to_string(INPUT_FILE).expect("File does not exist {INPUT_FILE}");
    let mut dsp: SegmentDisplay = [[0; SegLine]; SegCols];

    for i in inp.lines() {
        let tok = i
            .split(&['=', ' ', 'x'])
            .filter(|&x| x.len() > 0)
            .collect::<Vec<_>>();
        match tok[0] {
            "rect" => turn_on(
                &mut dsp,
                tok[1].parse::<usize>().unwrap(),
                tok[2].parse::<usize>().unwrap(),
            ),
            "rotate" => match tok[1] {
                "column" => shift_col_n(
                    &mut dsp,
                    tok[2].parse::<usize>().unwrap(),
                    tok[4].parse::<usize>().unwrap(),
                ),
                "row" => shift_row_n(
                    &mut dsp,
                    tok[3].parse::<usize>().unwrap(),
                    tok[5].parse::<usize>().unwrap(),
                ),
                _ => panic!("Command unknown"),
            },
            _ => panic!("Command unknown"),
        }
    }
    print_letters(&dsp);
    0
}
fn main() {
    const INPUT_FILE: &str = "input";
    println!("Part 1 output = {}", Part1(INPUT_FILE));
    println!("Part 2 output = {}", Part2(INPUT_FILE));
}
