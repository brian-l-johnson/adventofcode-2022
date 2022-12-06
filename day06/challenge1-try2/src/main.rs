use std::io;

fn no_dups(s: &str) -> bool{
    for i in 0..s.len() {
        for j in 0..s.len() {
            if i != j {
                if s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() {
                    return false;
                }
            }
        }
    }
    return true;
}

fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();

        let ws = 4;
        let mut s = 0;
        let mut e = ws; 

        while e < line_str.len() {
            let window = &line_str[s..e];
            if no_dups(window) {
                println!("{}", e);
                break;
            }
            s+=1;
            e+=1;
        }
    }
}
