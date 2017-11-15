extern crate rand;
use rand::{thread_rng, sample};
use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    let mut rng = thread_rng();
    let n = 26;
    let m = 4;
    let mut f = BufWriter::new(fs::File::create("machinglist").unwrap());
    for me in 1..n {
        let mut sample = sample(&mut rng, (1..n).filter(|&x| x != me), m)
            .iter()
            .fold(String::new(), |acc, &x| acc + &*x.to_string() + " ");
        sample.pop();
        sample = sample + "\n";
        let b = sample.as_bytes();
        f.write(b).unwrap();
    }
}
