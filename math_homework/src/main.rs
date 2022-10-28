use std::io;
fn main() {
    let mut new_line = String::new();
    io::stdin()
        .read_line(&mut new_line)
        .expect("Unable to read line");
    if new_line.ends_with("\n") {
        new_line.pop();
    }
    let values: Vec<i32> = new_line
        .split(' ')
        .filter(|w| w.len() > 0)
        .map(|w| w.parse().unwrap())
        .collect();
    let result = solution(values[0], values[1], values[2], values[3]);
    if result == vec![] {
        println!("impossible");
    } else {
        for v in result {
            println!("{} {} {}", v.0, v.1, v.2);
        }
    }
}

fn solution(b: i32, d: i32, c: i32, l: i32) -> Vec<(i32, i32, i32)> {
    let mut results: Vec<(i32, i32, i32)> = Vec::new();
    for x1 in 0..=250 {
        for x2 in 0..=250 {
            for x3 in 0..=250 {
                let diff = l - x1 * b - x2 * d - x3 * c;
                if diff < 0 {
                    break;
                }
                if diff == 0 {
                    results.push((x1, x2, x3));
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod acm_icpc_intro_to_cp3_test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            solution(2, 4, 4, 14),
            vec![
                (1, 0, 3),
                (1, 1, 2),
                (1, 2, 1),
                (1, 3, 0),
                (3, 0, 2),
                (3, 1, 1),
                (3, 2, 0),
                (5, 0, 1),
                (5, 1, 0),
                (7, 0, 0)
            ]
        );
        assert_eq!(solution(1, 1, 1, 0), vec![(0, 0, 0)]);
        assert_eq!(solution(2, 2, 2, 1), vec![]);
        assert_eq!(
            solution(100, 80, 60, 240),
            vec![(0, 0, 4), (0, 3, 0), (1, 1, 1)]
        );
        assert_eq!(solution(1, 1, 1, 1), vec![(0, 0, 1), (0, 1, 0), (1, 0, 0)]);
    }
}
