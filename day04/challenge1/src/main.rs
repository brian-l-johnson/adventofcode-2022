use std::io;

struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn full_overlap(&self, other: &Assignment) -> bool {
        println!("comparing {}-{} to {}-{}", self.start, self.end, other.start, other.end);
        self.start <= other.start && self.end >= other.end
    }
}

fn build_assignment(assiment_string: String) -> Assignment {
    let assigment_def: Vec<&str> = assiment_string.split('-').collect();
    Assignment {
        start: assigment_def[0].parse().unwrap(),
        end: assigment_def[1].parse().unwrap(),
    }
}

fn main() {
    let lines = io::stdin().lines();
    let mut overlap_count = 0;
    for line in lines {
        let assignment_line = line.unwrap();
        let play: Vec<&str>  = assignment_line.split(',').collect();

        let a1 = build_assignment(play[0].to_string());
        let a2 = build_assignment(play[1].to_string());
        if a1.full_overlap(&a2) || a2.full_overlap(&a1) {
            println!("full overlap found");
            overlap_count += 1
        }
    }
    println!("{} full overlaps found", overlap_count);
}
