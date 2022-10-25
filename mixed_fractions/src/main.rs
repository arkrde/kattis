use std::io;
fn main() {
    loop {
        let mut next_line: String = String::new();
        io::stdin()
            .read_line(&mut next_line)
            .expect("Unable to read line");
        if next_line.ends_with('\n') {
            next_line.pop();
        }
        let values: Vec<i32> = next_line
            .split(' ')
            .filter(|w| w.len() > 0)
            .map(|w| w.parse().unwrap())
            .collect();
        if values[0] == 0 && values[1] == 0 {
            break;
        }
        println!("{}", solution(values[0], values[1]));
    }
}

fn solution(num: i32, denom: i32) -> String {
    format!("{} {} / {}", num / denom, num % denom, denom)
}

#[cfg(test)]
mod csumb_fa22_practice_contest_6_test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(solution(27, 12), "2 3 / 12");
        assert_eq!(solution(2460000, 98400), "25 0 / 98400");
        assert_eq!(solution(3, 4000), "0 3 / 4000");
        assert_eq!(solution(3, 12), "0 3 / 12");
    }
}
