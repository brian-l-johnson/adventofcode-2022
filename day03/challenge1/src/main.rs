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

    for line in lines {
        let pack_line = line.unwrap();
        let pack_size = pack_line.len();

        let compartment1 = &pack_line[0..(pack_size/2)];
        let compartment2 = &pack_line[(pack_size/2)..];

        println!("{}:{}", compartment1, compartment2);

        let mut items = HashMap::new();

        for c in compartment1.chars() {
            items.insert(c, true);
        }
        for c in compartment2.chars()  {
            if items.contains_key(&c) {
                println!("{} is in both compartments", c);
                total = total + letter_scores.get(&c).unwrap();
                break;
            }
        }

    }
    println!("score: {total}");
}
