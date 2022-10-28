use std::io;
fn main() {
    loop {
        let mut new_line = String::new();
        io::stdin()
            .read_line(&mut new_line)
            .expect("Unable to read line");
        if new_line.ends_with("\n") {
            new_line.pop();
        }
        if new_line == "" {
            break;
        }
        let values: Vec<i32> = new_line
            .split(' ')
            .filter(|w| w.len() > 0)
            .map(|w| w.trim().parse().unwrap())
            .collect();
        println!("{}", solution(&values));
    }
}

fn solution(seq: &[i32]) -> i32 {
    seq.iter().sum::<i32>() / 2
}

#[cfg(test)]
mod acm_icpc_intro_to_cp3_test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(solution(&vec![1, 2, 3]), 3);
        assert_eq!(solution(&vec![2, 1, -1, 1, -1, 2]), 2);
    }
}
