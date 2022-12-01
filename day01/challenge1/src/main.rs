use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut total = 0;
    let mut max_elf = 0;

    for line in lines {
        match line.unwrap().trim().parse::<u32>() {
            Ok(num) => {
                println!("has food value {num}");
                total = total + num;
                num
            }
            Err(_) => {
                println!("elf had total: {total}");
                if total > max_elf {
                    max_elf = total;
                }
                total = 0;
                continue
            }
        };
    }
    println!{"max elf calorie count: {max_elf}"};
}
