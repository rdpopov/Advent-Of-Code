use std::vec::IntoIter;

struct Sequence {
    seed: Vec<u8>,
    sep_terator: Stack,
    symbol_iterator: IntoIter<u8>,
    nth_sequnce: usize,
}

impl Sequence {
    fn new(seed: Vec<u8>) -> Self {
        Self {
            seed: seed.to_owned(),
            sep_terator: Stack::new(),
            symbol_iterator: seed.into_iter(),
            nth_sequnce: 0,
        }
    }

    fn new_str(_seed: &str) -> Self {
        let seed = _seed
            .as_bytes()
            .into_iter()
            .map(|f| f - '0' as u8)
            .collect::<Vec<u8>>();
        Self {
            seed: seed.to_owned(),
            sep_terator: Stack::new(),
            symbol_iterator: seed.into_iter(),
            nth_sequnce: 0,
        }
    }
}

struct Stack {
    stack: Vec<(i32, i32)>,
    max_depth: i32,
}

impl Stack {
    fn new() -> Self {
        Self {
            stack: vec![(0, 0)],
            max_depth: 0,
        }
    }
}

impl Iterator for Stack {
    type Item = i32;

    // an infinite Sequence generator to generate the bit from the counter needed. the bit needed
    // is 1 + result
    //

    fn next(&mut self) -> Option<Self::Item> {
        let (depth, children) = self.stack.pop()?;
        if self.stack.len() == 0 {
            self.stack.push((self.max_depth, 1));
            self.max_depth += 1;
            self.stack.push((self.max_depth, 0));
        }
        if children == 0 || depth == 0 {
            return Some(depth);
        } else {
            self.stack.push((depth - 1, 2));
            self.stack.push((depth, 0));
            self.stack.push((depth - 1, 2));
        }
        return self.next();
    }
}
impl Iterator for Sequence {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: Self::Item;
        match self.symbol_iterator.next() {
            Some(x) => res = x,
            None => {
                let bit = self.sep_terator.next().unwrap();
                res = if (self.nth_sequnce & 1 << (bit + 1)) != 0 {
                    1
                } else {
                    0
                };
                self.nth_sequnce += 1;
                if self.nth_sequnce % 2 == 0 {
                    self.symbol_iterator = self.seed.clone().into_iter();
                } else {
                    self.symbol_iterator = self
                        .seed
                        .to_owned()
                        .into_iter()
                        .rev()
                        .map(|f| match f {
                            0 => 1,
                            1 => 0,
                            _ => 0,
                        })
                        .collect::<Vec<_>>()
                        .into_iter();
                }
            }
        }
        Some(res)
    }
}
fn hash_sequnce(seq:&mut Vec<u8>) {
    for _i in 0..(seq.len()/2){
        let i = _i *2;
        let j = i +1;
        seq[_i] = (seq[i] == seq[j]) as u8;
    }
    seq.resize(seq.len()/2,0);
}
fn Part1 (seed:&str,length:usize) -> String{
    let mut seq: Sequence = Sequence::new_str(seed);
    let mut ans = seq.take(length).collect::<Vec<u8>>();
    while &ans.len() % 2 == 0 {
        hash_sequnce(&mut ans);
    }
    ans.into_iter().map(|f| (f + '0' as u8) as char).collect()
}

fn main() {
    println!("Part 1 {}",Part1("11011110011011101", 272));
    println!("Part 2 {}",Part1("11011110011011101", 35_651_584));
}

#[test]
fn test_Part1() {
    const input: &str = "11011110011011101" ;
    const input_len: usize = 272;
    const expected: &str = "00000100100001100" ;
    let res = Part1(input, input_len);
    assert!(res == expected, "\n{res}\n{expected}");
}

#[test]
fn test_Part2() {
    const input: &str = "11011110011011101" ;
    const input_len: usize = 35_651_584;
    const expected: &str = "00011010100010010" ;
    let res = Part1(input, input_len);
    assert!(res == expected, "\n{res}\n{expected}");
}

