// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    let number = 600851475143;
    let answer = factorize(number).pop();
    if answer.is_some() {
        println!(
            "The largest prime factor of {} is '{}'",
            number,
            answer.unwrap()
        );
    } else {
        println!("Something went wrong.");
    }
}

fn factorize(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut divider = 2;
    let mut remainder = n;

    while remainder > 1 {
        if remainder % divider == 0 {
            factors.push(divider);
            println!("{}", divider);
            remainder /= divider;
        } else {
            divider += 1;
        }
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorize_some_numbers() {
        assert_eq!(factorize(1), []);
        assert_eq!(factorize(2), [2]);
        assert_eq!(factorize(3), [3]);
        assert_eq!(factorize(4), [2, 2]);
        assert_eq!(factorize(5), [5]);
        assert_eq!(factorize(6), [2, 3]);
        assert_eq!(factorize(7), [7]);
        assert_eq!(factorize(8), [2, 2, 2]);
        assert_eq!(factorize(9), [3, 3]);

        assert_eq!(factorize(13195), [5, 7, 13, 29])
    }
}
