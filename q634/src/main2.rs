fn num_triangular_sum(n: u64) -> u64 {
    let k = (-0.5 + (2.0 * (n as f64) + 0.25).sqrt()).floor() as u64;
    let near_triangular = k * (k + 1) / 2;
    if n == near_triangular {
        return 1;
    }
    let lst: Vec<u64> = (1..k + 1).map(|x| x * (x + 1) / 2).collect();
    for i in lst.iter() {
        if lst.iter().find(|&&x| x == n - i).is_some() {
            return 2;
        }
    }
    3
}

fn main() {
    assert_eq!(num_triangular_sum(1), 1);
    assert_eq!(num_triangular_sum(2), 2);
    assert_eq!(num_triangular_sum(3), 1);
    assert_eq!(num_triangular_sum(5), 3);
    assert_eq!(num_triangular_sum(1000000), 2);
}
