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
            Err(_) => {
                println!("elf had total: {total}");
                totals.push(total);
                total = 0;
                continue
            }
        };
    }
    totals.push(total);

    totals.sort();
    println!("total number of eolves {}",totals.len());
    let mut sum = 0;
    for n in totals.len()-3..totals.len() {
        println!("elf {} has {} calories", n, totals[n]);
        sum = sum + totals[n];
    }
    println!{"top 3 elves have {sum} calories"};
}
