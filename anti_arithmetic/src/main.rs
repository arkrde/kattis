use std::io;
fn main() {
    loop {
        let mut next_line = String::new();
        io::stdin()
            .read_line(&mut next_line)
            .expect("unable to read new line");
        if next_line.ends_with("\n") {
            next_line.pop();
        }
        if next_line == "0" {
            break;
        }
        let mut words = next_line.split(' ');
        words.nth(0);
        let values: Vec<usize> = words
            .map(|w| w.trim())
            .filter(|w| w.len() > 0)
            .map(|w| w.parse().unwrap())
            .collect();
        if solution(&values) == true {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn solution(seq: &[usize]) -> bool {
    let mut data = Vec::from(seq);
    let mut shifts: Vec<usize> = vec![0; seq.len()];
    let n = data.len();
    for i in 1..n {
        let d = data[i];
        let mut j = i;
        while j > 0 && d < data[j - 1] {
            data[j] = data[j - 1];
            j -= 1;
            shifts[d] += 1;
        }
        data[j] = d;
    }
    for i in 1..n - 1 {
        if 2 * shifts[i] == shifts[i - 1] + shifts[i + 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod cp_csunsa_03 {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(solution(&[0, 2, 1]), true);
        assert_eq!(solution(&[2, 0, 1, 3, 4]), false);
        assert_eq!(solution(&[2, 4, 3, 5, 0, 1]), true);
    }
}
