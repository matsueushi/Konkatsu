extern crate rand;

fn main() {
    let total=100000;
    let mut count=0;
    for i in 0..total{

    let a = rand::random::<f64>();
    let b = rand::random::<f64>();
    if a*a + b*b <= 1. {
        count=count+1;
    }
    }
    let pi = 4.*count as f64 /total as f64;
    println!("pi={}", pi);
}
