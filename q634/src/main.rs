use std::f64;

//fn triangle(k : i64) -> i64{
//    return (k * (k + 1)) / 2;
//}
//fn is_even(x: i64) -> bool {
//    if x % 2 == 0 { true } else { false }
//}

fn isTriangle(num : i64) -> bool{
    let x = (1.0+(1.0 + 8.0 * (num as f64)).sqrt())/2.0;
    return x.floor() == x;
}
fn main() {

    let N = 999999998;

    if(isTriangle(N)){
        println!("1");
    }
    else{
        let half = N / 2;
        let mut num = 3;
        for i in half..N {
            let subtracted = N - i;
            if(isTriangle(subtracted) && isTriangle(i)){
                println!("{},{}",i,subtracted);
                num = 2;
                break;
            }
        }
        println!("{}",num);
    }
}
