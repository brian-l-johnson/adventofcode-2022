use std::io;
use regex::Regex;

fn main() {
    let stack_re = Regex::new(r"^\s*(\[[A-Z]\]\s)+").unwrap();
    let index_re = Regex::new(r"^[\s\d]*$").unwrap();
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let lines = io::stdin().lines();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut stacks_inited = false;

    for line in lines {
        let line_str = line.unwrap();
        if stack_re.is_match(&line_str) {
            let len = line_str.len();
            if !stacks_inited {
                let mut i = 0;
                while (i*4)+1 < len {
                    let v: Vec<char> = Vec::new();
                    stacks.push(v);
                    i+=1;
                }
                stacks_inited = true;
            }
            let mut i = 0;
            while (i*4)+1 < len {
                let crate_id = line_str.chars().nth((i*4)+1).unwrap();
                if crate_id != ' ' {
                    stacks[i].insert(0, crate_id);
                }
                i += 1;
            }
        }
        else if line_str == "" {
            //println!("found blank line");
        }
        else if index_re.is_match(&line_str) {
            //println!("found index line");
        }
        else if move_re.is_match(&line_str) {
            let inst = move_re.captures(&line_str).unwrap();
            let count:u32 = inst[1].parse().unwrap();
            let src:usize = (inst[2].parse::<usize>().unwrap())-1;
            let dst:usize = (inst[3].parse::<usize>().unwrap())-1;
            for _i in 0..count {
                let c = stacks[src].pop().unwrap();
                stacks[dst].push(c);
            }
        }
        else {
            println!("{:?}", stacks);
            panic!("unrecognized line format");
        }
    }
    println!("{:?}", stacks);
    

    let mut answer = String::new();
    for mut stack in stacks {
        let c = stack.pop().unwrap();
        answer.push(c)
    }
    println!("{answer}");

}
