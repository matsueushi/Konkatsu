extern crate rand;
use rand::{thread_rng, sample};
use std::fs;
use std::io::{BufWriter, Write};

fn main(){
    let mut rng = thread_rng();
    let n = 26;
    let m = 3;
    let mut f = BufWriter::new(fs::File::create("machinglist").unwrap());
    for _ in 1..n{
        let sample = sample(&mut rng, 1..n, m);
        let a1 = sample[0].to_string();
        let a2 = sample[1].to_string();
        let a3 = sample[2].to_string();
        let a4 = "\n".to_string();
        let a_sum = [a1, a2, a3, a4].join(" ");
        let b = a_sum.as_bytes();
        f.write(b).unwrap();
    }
}
