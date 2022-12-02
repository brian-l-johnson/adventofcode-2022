use std::io;

fn calc_score(o: &str, u: &str) -> u32 {
    let mut score: u32;
    match u {
        "X" => {
            println!("Need: lose");
            score = 0;
            match o {
                "A" => {
                    println!("Opponent: rock");
                    println!("You: scissors");
                    score = score + 3;
                }
                "B" => {
                    println!("Opponent: paper");
                    println!("You: rock");
                    score = score + 1;
                }
                "C" => {
                    println!("Opponent: scissors");
                    println!("You: paper");
                    score = score + 2;
                }
                _ => {
                    panic!("illegal play");
                }
            }
        }
        "Y" => {
            println!("Need: draw");
            score = 3;
            match o {
                "A" => {
                    println!("Opponent: rock");
                    score = score + 1;
                }
                "B" => {
                    println!("Opponent: paper");
                    score = score + 2; 
                }
                "C" => {
                    score = score + 3;
                    println!("Opponent: scissors");
                }
                _ => {
                    panic!("illegal play");
                }
            }
        }
        "Z" => {
            println!("Need win");
            score = 6;
            match o {
                "A" => {
                    println!("Opponent: rock");
                    println!("You: Paper");
                    score = score + 2;
                }
                "B" => {
                    println!("Opponent: paper");
                    println!("You: scissors");
                    score = score + 3;
                }
                "C" => {
                    println!("Opponent: scissors");
                    println!("You: rock");
                    score = score + 1;
                }
                _ => {
                    panic!("illegal play");
                }
            }
        }
        _ => {
            panic!("illegal play");
        }
    }
    println!("game score: {}", score);
    score
}

fn main() {
    let lines = io::stdin().lines();

    let mut score = 0;

    for line in lines {
        let play_line = line.unwrap();
        let play: Vec<&str>  = play_line.split_whitespace().collect();
        //println!("{}", &play[0]);
        score = score + calc_score(play[0], play[1]);
    }
    println!("score is: {}", score);
}
