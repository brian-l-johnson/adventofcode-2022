use std::io;
use std::collections::HashMap;


fn main() {

    let letter_scores = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15), 
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    let lines = io::stdin().lines();

    let mut total = 0;
    let mut i = 0;
    let mut items = HashMap::new();

    for line in lines {
        let pack_line = line.unwrap();
        let mut item_vect: Vec<char> = pack_line.chars().collect();
        item_vect.sort();
        item_vect.dedup();
    
        for c in item_vect {
            let count = items.entry(c).or_insert(0);
            *count += 1;
        }
        i += 1;
        if i == 3 {
            i = 0;
            for (key, value) in &items {
                if *value == 3 {
                    println!("group badge: {}", key);
                    total += letter_scores.get(&key).unwrap();
                }
            }
            items.clear();
        }
    }
    println!("score: {total}");
}
