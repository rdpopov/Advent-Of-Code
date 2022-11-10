use md5;
use std::thread;
fn mc5(low:u32,high:u32) -> thread::JoinHandle<()> {
         thread::spawn(move ||{
            for i in low..high{
                let hash = format!("{:x}",md5::compute(format!("ckczppom{}",i)));
                if &hash[..5] == "00000"{
                    println!("lowest number is {}",i);
                    break;
                }
            }
        })
}
fn mc6(low:u32,high:u32) -> thread::JoinHandle<()> {
         thread::spawn(move ||{
            for i in low..high{
                let hash = format!("{:x}",md5::compute(format!("ckczppom{}",i)));
                if &hash[..6] == "000000"{
                    println!("lowest number is {}",i);
                    break;
                }
            }
        })
}

fn main() {
    let seed = "ckczppom";
    let has = format!("{:x}",md5::compute(format!("ckczppom{}",10)));

    let b = [
        0,
        u32::MAX / 4,
        u32::MAX / 2,
        (u32::MAX / 4 ) *3,
        u32::MAX ];

    // let hdls:[thread::JoinHandle<()>;4] =  [
    //     mc5(b[0],b[1]),
    //     mc5(b[1],b[2]),
    //     mc5(b[2],b[3]),
    //     mc5(b[3],b[4]),
    // ];

    let hdls:[thread::JoinHandle<()>;4] =  [
        mc6(b[0],b[1]),
        mc6(b[1],b[2]),
        mc6(b[2],b[3]),
        mc6(b[3],b[4]),
    ];
    for i in hdls.into_iter(){
        i.join();
    }
}
