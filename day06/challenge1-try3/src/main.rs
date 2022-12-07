use std::io;

fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();

        let ws = 14;
        let mut s = 0;
        //let mut e = ws; 

        while s+ws < line_str.len() {
            let window = &line_str[s..s+ws];
            let mut wv:Vec<char> = window.chars().collect();
            wv.sort();
            wv.dedup();
            if wv.len() == ws {
                println!("{}", s+ws);
                break;
            }
            s+=1;
            //e+=dup_offset;
        }
    }
}
