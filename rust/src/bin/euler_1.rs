fn sum_3_5_dividers_to_1000() -> u64 {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum = sum + n;
        }
    }
    sum
}

fn main() {
    println!("Euler 1: {}", sum_3_5_dividers_to_1000());
}
