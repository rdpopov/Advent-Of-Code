use md5::Digest;
use std::sync::mpsc;
use std::thread::spawn;

const THREADS: u64 = 8;

fn Part1(_seed: &str, tail_length: usize) -> String {
    let mut base: u64 = 0;
    const INTERVAL_LENGTH: u64 = 4_000_000;
    let mut numbers: Vec<u64> = vec![];
    let (tx, rx) = mpsc::channel::<u64>();
    while numbers.len() < tail_length {
        let mut th_pool = vec![];
        for i in 0..THREADS {
            let from = base + i * INTERVAL_LENGTH;
            let to = base + (i + 1) * INTERVAL_LENGTH;
            let _sender: mpsc::Sender<u64> = tx.clone();
            let seed = _seed.to_owned();
            let hndl = spawn(move || {
                let sender = _sender;
                for j in from..to {
                    let hsh: Digest = md5::compute(format!("{}{}", seed, j));
                    if 0 == hsh[0] && 0 == hsh[1] && 0 == (hsh[2] & 0xF0) {
                        match sender.send(j as u64) {
                            Ok(_) => (),
                            Err(_) => println!("Couldn't send value {} ", i),
                        };
                    }
                }
            });
            th_pool.push(hndl);
        }

        base += THREADS * INTERVAL_LENGTH;
        for i in th_pool {
            i.join().unwrap();
        }

        rx.try_iter().for_each(|i| {
            numbers.push(i);
        });
    }

    numbers.sort();
    return numbers
        .into_iter()
        .take(tail_length)
        .map(|x| format!("{:x}", md5::compute(format!("{}{}", _seed, x))[2]))
        .collect::<String>();
}

fn construct_password(_seed: &str, numbers: &Vec<u64>) -> String {
    let mut result = "        ".to_string();
    let pairs: Vec<(u8, u8)> = numbers
        .into_iter()
        .map(|x| {
            let hsh = md5::compute(format!("{}{}", _seed, x));
            (hsh[2], (hsh[3] >> 4) & 0x0F)
        })
        .collect();
    for i in pairs {
        let idx: usize = i.0 as usize;
        let value = i.1;
        if let Some(x) = result.chars().nth(idx) {
            if ' ' == x {
                let symbol = format!("{:x}", value);
                result.replace_range(idx..(idx + 1), &symbol[0..1]);
            }
        }
    }
    result
}

fn Part2(_seed: &str, tail_length: usize) -> String {
    let mut base: u64 = 0;
    const INTERVAL_LENGTH: u64 = 4_000_000;
    let mut numbers: Vec<u64> = vec![];
    let (tx, rx) = mpsc::channel::<u64>();
    let mut result: String = "        ".to_string();
    // repeat while we have empty space in the array
    while result.contains(" ") {
        let mut th_pool = vec![];
        for i in 0..THREADS {
            let from = base + i * INTERVAL_LENGTH;
            let to = base + (i + 1) * INTERVAL_LENGTH;
            let _sender: mpsc::Sender<u64> = tx.clone();
            let seed = _seed.to_owned();
            let hndl = spawn(move || {
                let sender = _sender;
                for j in from..to {
                    let hsh: Digest = md5::compute(format!("{}{}", seed, j));
                    if 0 == hsh[0] && 0 == hsh[1] && 0 == (hsh[2] & 0xF0) {
                        match sender.send(j as u64) {
                            Ok(_) => (),
                            Err(_) => println!("Couldn't send value {} ", i),
                        };
                    }
                }
            });
            th_pool.push(hndl);
        }

        base += THREADS * INTERVAL_LENGTH;
        for i in th_pool {
            i.join().unwrap();
        }
        rx.try_iter().for_each(|i| {
            numbers.push(i);
        });

        numbers.sort();
        result = construct_password(_seed, &numbers);
        println!("{:?}", result);
    }
    result
}

fn main() {
    println!("{}", Part1("abc",8));
    println!("{}", Part1("uqwqemis",8));
    println!("{}", Part2("abc",8));
    println!("{}", Part2("uqwqemis",8));
}

mod test {
    use super::*;

    #[test]
    fn test_input_part1() {
        let generated = Part1("abc", 8);
        let expected = "18f47a30";
        assert!(expected == generated, "result {generated} != {expected}");
    }

    #[test]
    fn real_input_part1() {
        let generated = Part1("uqwqemis", 8);
        let expected = "1a3099aa";
        assert!(expected == generated, "result {generated} != {expected}");
    }

    #[test]
    fn test_input_part2() {
        let generated = Part2("abc", 8);
        let expected = "05ace8e3";
        assert!(expected == generated, "result {generated} != {expected}");
    }

    #[test]
    fn real_input_part2() {
        let generated = Part2("uqwqemis", 8);
        let expected = "694190cd";
        assert!(expected == generated, "result {generated} != {expected}");
    }
}
