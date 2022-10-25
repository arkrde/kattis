use std::io;
fn main() {
    let mut new_line = String::new();
    io::stdin()
        .read_line(&mut new_line)
        .expect("unable to read line");
    let n: i32 = new_line.trim().parse().unwrap();
    println!("{}", solution(n));
}

fn solution(n: i32) -> i32 {
    if n < 100 {
        99
    } else if n % 100 == 0 {
        n - 1
    } else {
        let m = n / 100;
        if (n - m * 100 + 1) < (m * 100 + 99 - n) {
            m * 100 - 1
        } else {
            m * 100 + 99
        }
    }
}

#[cfg(test)]
mod csumb_fa22_practice_contest_6_test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(solution(10), 99);
        assert_eq!(solution(249), 299);
        assert_eq!(solution(10000), 9999);
    }
}
