extern crate rand;
use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    let n = 26;
    let m = 4;
    let mut f = BufWriter::new(fs::File::create("machinglist").unwrap());
    for _ in 1..n {
        let mut sample: Vec<u32> = Vec::new();
        while sample.len() != m {
            let r = rand::random::<u32>() % n as u32;
            if !sample.contains(&r) {
                sample.push(r);
            }
        }
        let mut lst = sample.iter().fold(String::new(), |acc, &x| {
            acc + &*x.to_string() + " "
        });
        lst.pop();
        lst = lst + "\n";
        let b = lst.as_bytes();
        f.write(b).unwrap();
    }
}
