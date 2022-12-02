use std::io;

fn calc_score(o: &str, u: &str) -> u32 {
    let mut score: u32;
    match u {
        "X" => {
            println!("You: rock");
            score = 1;
            match o {
                "A" => {
                    println!("Opponent: rock");
                    println!("tie");
                    score = score + 3;
                }
                "B" => {
                    println!("Opponent: paper");
                    println!("lose");
                    score = score + 0;
                }
                "C" => {
                    println!("Opponent: scissors");
                    println!("win");
                    score = score + 6;
                }
                _ => {
                    panic!("illegal play");
                }
            }
        }
        "Y" => {
            println!("You: paper");
            score = 2;
            match o {
                "A" => {
                    println!("Opponent: rock");
                    println!("win");
                    score = score + 6;
                }
                "B" => {
                    println!("Opponent: paper");
                    println!("tie");
                    score = score + 3; 
                }
                "C" => {
                    score = score + 0;
                    println!("lose");
                    println!("Opponent: scissors");
                }
                _ => {
                    panic!("illegal play");
                }
            }
        }
        "Z" => {
            println!("You: scissors");
            score = 3;
            match o {
                "A" => {
                    println!("Opponent: rock");
                    println!("lose");
                    score = score + 0;
                }
                "B" => {
                    println!("Opponent: paper");
                    println!("win");
                    score = score + 6;
                }
                "C" => {
                    println!("Opponent: scissors");
                    println!("tie");
                    score = score + 3;
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
