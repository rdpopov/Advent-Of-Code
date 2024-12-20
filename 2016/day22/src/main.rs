use std::{fs, iter::Empty, cmp::max};

const USED: usize = 2;
const AVAIL: usize = 3;

#[test]
fn test_part1(){
    assert_eq!(1003, part1("input_str"));
}
fn part1(input_file: &str) -> usize {
    let inp = fs::read_to_string(input_file).expect("cant acess file");
    let mut sizes: Vec<(i32, i32)> = vec![];

    for ln in inp.lines() {
        let tmp: Vec<&str> = ln.split(&[' ', 'T']).filter(|&x| x.len() > 0).collect();
        let pair = (
            tmp[AVAIL].parse::<i32>().expect("has to be a number"),
            tmp[USED].parse::<i32>().expect("has to be a number"),
        );
        sizes.push(pair);
    }
    let mut res: usize = 0;
    for i in 0..sizes.len() {
        for j in 0..sizes.len() {
            if i != j && sizes[i].0 > sizes[j].1 && sizes[j].1 != 0 {
                res += 1;
            }
        }
    }
    println!("{:?}", sizes);
    res
}

fn visuzlize (matr:&Vec<Vec<(i32,i32)>>,maxX:usize,maxY:usize,empty_size:i32) {
    for i in 0..maxY{
        for j in 0..maxX{
            if matr[i][j].1 == 0 {
                print!("{}","_");
            } else if matr[i][j].1 < matr[maxX][0].1 || matr[i][j].1 > empty_size {
                print!("{}","#");
            } else {
                print!("{}",".");
            }
        }
        print!("\n");
    }
}

// no need for a star . just count it .... and then its formulaic to move the data 1 pace and for
// the empty to be in the same position relative to it, it takes 5 moves =>
// and the final takes 2 moves because it doesnt need to be afull cycle.
// the total amount of moves would be the ((length-1)        -1)    *   5                     + 2            + 25  
//                                         we start here    for final   for each link moved    for the final   to move the empty in position
//
#[test]
fn test_part2(){
    assert_eq!(192, part2("input_str"));
}

fn part2(input_file: &str) -> usize {
    let inp = fs::read_to_string(input_file).expect("cant acess file");
    let mut sizes: Vec<Vec<(i32, i32)>> = vec![vec![(0,0);40];40];
    let mut empties: Vec<i32> = vec![];
    let mut maxX=35;
    let mut maxY=29;
    let mut empty_x_y = (0,0);
    let mut empty_size = 0;

    for ln in inp.lines() {
        let tmp: Vec<&str> = ln.split(&[' ', '-','y','x' ,'T']).filter(|&x| x.len() > 0).collect();
        let x = tmp[1].parse::<usize>().unwrap();
        let y = tmp[2].parse::<usize>().unwrap();
        maxX = max(x,maxX);
        maxY = max(y,maxY);

        let sz:i32  = tmp[3].parse().unwrap();
        let taken:i32  = tmp[4].parse().unwrap();
        if taken == 0{
            empty_x_y = (x,y) ;
            empty_size = sz;
        }
        println!("{:?}",tmp);
        sizes[y][x] = (sz,taken);
    }
    let move_almost_there = (maxX - 2 ) * 5 + 2;
    let movments = 25 + move_almost_there ;

    visuzlize(&sizes, maxX, maxY, empty_size);
    movments
}


fn main() {
    println!("Part 1 {}", part1("input_str"));
    println!("Part 2 {}", part2("input_str"));
}
