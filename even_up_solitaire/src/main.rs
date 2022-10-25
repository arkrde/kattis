use std::io;
fn main() {
    let mut next_line = String::new();
    io::stdin()
        .read_line(&mut next_line)
        .expect("unable to read next line");
    if next_line.ends_with('\n') {
        next_line.pop();
    }
    let _: i32 = next_line.trim().parse().unwrap();
    io::stdin()
        .read_line(&mut next_line)
        .expect("unable to read next line");
    if next_line.ends_with('\n') {
        next_line.pop();
    }
    let values: Vec<i32> = next_line
        .split(' ')
        .filter(|w| w.len() > 0)
        .map(|w| w.parse().unwrap())
        .collect();
    println!("{}", solution(&values));
}

fn solution_impl(seq: &Vec<i32>) -> usize {
    if seq.len() == 0 {
        0
    } else {
        let mut start: usize = 0;
        let mut ops: usize = 0;

        let mut new_vec: Vec<i32> = vec![];

        while start < (seq.len() - 1) {
            if (seq[start] + seq[start + 1]) % 2 == 0 {
                start += 2;
                ops += 1;
            } else {
                new_vec.push(seq[start]);
                start += 1;
            }
        }
        if start == seq.len() - 1 {
            new_vec.push(seq[start]);
        }
        if ops == 0 {
            0
        } else {
            ops + solution_impl(&new_vec)
        }
    }
}

fn solution(seq: &Vec<i32>) -> usize {
    seq.len() - 2 * solution_impl(&seq)
}

#[cfg(test)]
mod csumb_fa22_practice_contest_6_test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(solution(&vec![1]), 1);
        assert_eq!(solution(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
        assert_eq!(solution(&vec![1, 3, 3, 4, 2, 4, 1, 3, 7, 1]), 2);
        assert_eq!(solution(&vec![1, 3, 2, 4,]), 0);
        assert_eq!(solution(&vec![1, 1, 1, 1]), 0);
        assert_eq!(solution(&vec![2, 2, 2, 2]), 0);
        assert_eq!(solution(&vec![1, 1, 1, 1, 1]), 1);
        assert_eq!(solution(&vec![1, 1, 2, 2, 1]), 1);
        assert_eq!(solution(&vec![1, 1, 2, 1, 1, 2]), 0);
        assert_eq!(solution(&vec![1, 1, 1, 2, 1, 1, 1, 2]), 4);
    }
}
