use std::io;
use std::collections::VecDeque;

#[derive(Debug)]
struct RingBuffer {
    inner: VecDeque<char>,
    capacity: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity: capacity,
        }
    }
    
    pub fn push(&mut self, item: char) {
        if self.inner.len() == self.capacity {
            self.inner.pop_front();
            self.inner.push_back(item);
            debug_assert!(self.inner.len() == self.capacity);
        } else {
            self.inner.push_back(item);
        }
        //println!("{:?}", self.inner);
    }
    
    pub fn all_unique(&mut self) -> bool {
        println!("{:?}", self.inner);
        for i in 0..self.capacity {
            for j in 0..self.capacity {
                if i!=j {
                    if self.inner[i] == self.inner[j] {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}


fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();

        let mut buf = RingBuffer::new(4);
        
        for (i, c) in line_str.chars().enumerate() {
            buf.push(c);
             
            if i>3 {
                if buf.all_unique() {
                    println!("{}", i+1);
                    break;
                }
            }
        }
    }  
}
