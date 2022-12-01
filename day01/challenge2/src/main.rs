use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut total = 0;
    let mut totals: Vec<u32> = Vec::new();

    for line in lines {
        match line.unwrap().trim().parse::<u32>() {
            Ok(num) => {
                println!("has food value {num}");
                total = total + num;
                num
            }
            Err(_) => { //if its not a number assume its a new line and we are on a new elf.  this is bad, but it works
                println!("elf had total: {total}");
                totals.push(total);
                total = 0;
                continue
            }
        };
    }
    totals.push(total); //add last elf, again bad

    totals.sort();
    println!("total number of elves {}",totals.len());
    let mut sum = 0;
    for n in totals.len()-3..totals.len() {
        println!("elf {} has {} calories", n, totals[n]);
        sum = sum + totals[n];
    }
    println!{"top 3 elves have {sum} calories"};
}
