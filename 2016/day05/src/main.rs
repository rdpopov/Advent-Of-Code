use std::sync::mpsc;
use std::thread::{self, Thread};


const THREADS:u64 = 8;
fn brute_froce(seed:&str,from:u64,to:u64,) -> &str {
    let mut idx:u64 = 0;
    let mut passwd:String = "".to_string();
    while passwd.len() < pass_len {
        let hsh = md5::compute(format!("{}{}",seed,idx));

        if 0 == hsh[0] && 0 == hsh[1] &&  0 == hsh[2] && 0 == hsh[3] && 0 == hsh[4] {
            println!("{:?}",hsh);
            break;
        }
        idx+=1;
    }


    seed
}

fn Part1(seed:&str) {
    let mut base:u64 = 0;
    let mut interval_length:u64 = 4_000_000;
    let mut numbers:Vec<u64> = vec![];
    let mut thread_pool:Vec<Option<Thread>> = vec![];
    thread_pool.resize(THREADS as usize, None);
    let (tx,rx) = mpsc::channel::<u64>();
    while numbers.len() < 8 {
        for i in 0..THREADS{
            let from = base + i*interval_length;
            let to = base + (i+1)*interval_length;
            thread::spawn(move || {
                let sender = tx.clone();
                for i in from..to {
                    let hsh = md5::compute(format!("{}{}",seed,i));
                    // if 0 == hsh[0] && 0 == hsh[1] &&  0 == hsh[2] && 0 == hsh[3] && 0 == hsh[4] {
                        sender.send(i as u64);
                    // }
                }
            });
        }
    }
    
    

}

fn main() {
    println!("{:?}",md5::compute(format!("{}{}","abc",3231929)));
    // brute_froce("abc", 8);
}
