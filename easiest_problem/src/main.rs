use std::io;
fn main() {
    loop {
        let mut next_line = String::new();
        io::stdin()
            .read_line(&mut next_line)
            .expect("unable to read line");
        if next_line.ends_with("\n") {
            next_line.pop();
        }
        let n: i64 = next_line.trim().parse().unwrap();
        if n == 0 {
            break;
        } else {
            println!("{}", solution(n));
        }
    }
}

fn sum_of_digits(n: i64) -> i64 {
    let mut sum = 0;
    let mut n = n;
    while n != 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn solution(n: i64) -> i64 {
    for i in 11.. {
        if sum_of_digits(n * i) == sum_of_digits(n) {
            return i;
        }
    }
    10
}

#[cfg(test)]
mod acm_icpc_intro_to_cp3_test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(solution(3029), 37);
        assert_eq!(solution(4), 28);
        assert_eq!(solution(5), 28);
        assert_eq!(solution(42), 25);
    }
}
