use std::io;

fn no_dups(s: &str) -> usize{
    for i in 0..s.len() {
        for j in 0..s.len() {
            if i != j {
                if s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() {
                    return i+1;
                }
            }
        }
    }
    return 0;
}

fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();

        let ws = 14;
        let mut s = 0;
        //let mut e = ws; 

        while s+ws < line_str.len() {
            let window = &line_str[s..s+ws];
            let dup_offset = no_dups(window);
            if dup_offset == 0 {
                println!("{}", s+ws);
                break;
            }
            s+=dup_offset;
            //e+=dup_offset;
        }
    }
}
