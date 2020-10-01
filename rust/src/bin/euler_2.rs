fn main() {
    // the sum will be in the order of 10M so it's safe to use u32, which is in order of 4G
    let mut sum: u32 = 0;
    let mut fibs: [u32; 3] = [0, 1, 2];

    while fibs[2] < 4000000 {
        sum += fibs[2];
        fibs = next_3_fibonacci_steps(fibs)
    }

    println!("Euler 2, sum of even fibonacci numbers < 4M: {}", sum);
}

// the sum of an uneven and even number is uneven, of two unevens even
// since we start with 1, 2 (uneven, even) we therefore know the following sets of 3 numbers in the fibsuence
// will always be (uneven, uneven, even)
fn next_3_fibonacci_steps(former: [u32; 3]) -> [u32; 3] {
    let next_0 = former[1] + former[2];
    let next_1 = former[2] + next_0;
    let next_2 = next_0 + next_1;
    [next_0, next_1, next_2]
}

#[cfg(test)]
mod test_fibonacci_generation {
    use super::*;

    #[test]
    fn a_set_of_3_fib_numbers_is_calculated_correctly() {
        assert_eq!(next_3_fibonacci_steps([1, 2, 3]), [5, 8, 13]);
    }

    #[test]
    fn input_does_not_have_to_be_valid_fibonacci_numbers_or_sequence() {
        assert_eq!(next_3_fibonacci_steps([7, 11, 0]), [11, 11, 22]);
    }
}
