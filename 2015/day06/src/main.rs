use read_lines::read_by_lines;

mod read_lines;
#[derive(Debug)]
enum CmdType {
    On = 0,
    Off,
    Toggle
}
#[derive(Debug)]
struct command {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    cmd:CmdType
}

impl command {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32, cmd: &str) -> Self { 
        let cmd = match cmd {
            "on" => CmdType::On,
            "off" => CmdType::Off,
            "toggle" => CmdType::Toggle,
            _ => CmdType::Toggle
        };
        Self { x0, y0, x1, y1, cmd }
    }
}

fn prep_cmd (l:&str) -> command {
    let mut cmd:Vec<&str> = l.split(" ").collect::<Vec<&str>>();
    if cmd[0] == "turn" {
        cmd.remove(0);
    }
    let xy0:Vec<i32> = cmd[1].split(",").map(|x| x.parse::<i32>().expect(&format!("trying to unwrap - {} -",x))).collect::<Vec<i32>>();
    let xy1:Vec<i32> = cmd[3].split(",").map(|x| x.parse::<i32>().expect(&format!("trying to unwrap - {} -",x))).collect::<Vec<i32>>();
    command::new(xy0[0], xy0[1], xy1[0], xy1[1], cmd[0])
}
fn exec_cmd_flip(matr:&mut Vec<Vec<bool>>, l:&str) {
    let cmd = prep_cmd(l);
    for x in cmd.x0 .. cmd.x1+1 {
        for y in cmd.y0 .. cmd.y1+1 {
            // println!("modify {} {}",x,y);
            match cmd.cmd {
                CmdType::Toggle => {matr[x as usize][y as usize] = !matr[x as usize][y as usize]}
                CmdType::On => {matr[x as usize][y as usize] = true }
                CmdType::Off => {matr[x as usize][y as usize] = false }
            }
        }
    }

}
fn exec_cmd_sum(matr:&mut Vec<Vec<i32>>, l:&str) {
    let cmd = prep_cmd(l);
    for x in cmd.x0 .. cmd.x1+1 {
        for y in cmd.y0 .. cmd.y1+1 {
            match cmd.cmd {
                CmdType::Toggle => {matr[x as usize][y as usize]+=2}
                CmdType::On => {matr[x as usize][y as usize]+=1}
                CmdType::Off => {
                    if matr[x as usize][y as usize] > 0 {
                        matr[x as usize][y as usize] -= 1;
                    }
                }
            }
        }
    }
}

fn count_value_flip(matr:&Vec<Vec<bool>>, l:bool) -> i32 {
    let mut res:i32 = 0;
    for line in matr.into_iter() {
        for v in line.into_iter() {
            if v == &l {
                res+=1;
            }
        }
    }
    res
}

fn count_value_sum(matr:&Vec<Vec<i32>>) -> i32 {
    let mut res:i32 = 0;
    for line in matr.into_iter() {
        for v in line.into_iter() {
            res += v;
        }
    }
    res
}


fn scenario_1() {
    let mut lamp:Vec<Vec<bool>> = vec![vec![false;1000];1000]; // 1000x1000 vector matrix
    if let Ok(lines) = read_by_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                exec_cmd_flip(&mut lamp, &l);
            }
        }
    }
    println!("scenario_1 lamps on {}",count_value_flip(&lamp, true));
}

fn scenario_2() {
    let mut lamp:Vec<Vec<i32>> = vec![vec![0;1000];1000]; // 1000x1000 vector matrix
    if let Ok(lines) = read_by_lines("./input") {
        for line in lines {
            if let Ok(l) = line {
                exec_cmd_sum(&mut lamp, &l);
            }
        }
    }
    println!(" scenario_2 lamps on {}",count_value_sum(&lamp));
}



fn main() {
    // let mut lamp:Vec<Vec<bool>> = vec![vec![false;1000];1000]; // 1000x1000 vector matrix
    // exec_cmd(&mut lamp, "turn off 499,499 through 500,500");
    scenario_1();
    scenario_2();
}
