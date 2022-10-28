use std::io;
fn main() {
    for i in 1.. {
        let mut next_line = String::new();
        io::stdin()
            .read_line(&mut next_line)
            .expect("unable to read line");
        if next_line.ends_with("\n") {
            next_line.pop();
        }
        if next_line == "" {
            break;
        }
        println!("Case {}:", i);
        let n: i32 = next_line.trim().parse().unwrap();
        let mut seq: Vec<i32> = vec![];
        for _ in 0..n {
            let mut next_line = String::new();
            io::stdin()
                .read_line(&mut next_line)
                .expect("unable to read line");
            if next_line.ends_with("\n") {
                next_line.pop();
            }
            let v: i32 = next_line.trim().parse().unwrap();
            seq.push(v);
        }
        let solution = Solution::new(&seq);
        let mut next_line = String::new();
        io::stdin()
            .read_line(&mut next_line)
            .expect("unable to read line");
        if next_line.ends_with("\n") {
            next_line.pop();
        }
        let m: i32 = next_line.trim().parse().unwrap();
        for _ in 0..m {
            let mut next_line = String::new();
            io::stdin()
                .read_line(&mut next_line)
                .expect("unable to read line");
            if next_line.ends_with("\n") {
                next_line.pop();
            }
            let q: i32 = next_line.trim().parse().unwrap();
            println!("Closest sum to {} is {}.", q, solution.solve(q));
        }
    }
}

struct Solution {
    grid: Vec<i32>,
}

impl Solution {
    fn new(seq: &[i32]) -> Solution {
        let mut grid: Vec<i32> = vec![];
        for i in 0..seq.len() {
            for j in (i + 1)..seq.len() {
                grid.push(seq[i] + seq[j]);
            }
        }
        Solution { grid }
    }
    fn solve(&self, q: i32) -> i32 {
        let pos = self
            .grid
            .iter()
            .enumerate()
            .min_by_key(|(_, v)| i32::abs(**v - q))
            .unwrap()
            .0;
        self.grid[pos]
    }
}

#[cfg(test)]
mod cp_csunsa_03 {
    use super::*;
    #[test]
    fn test() {
        let s = Solution::new(&[3, 12, 17, 33, 34]);
        assert_eq!(s.solve(1), 15);
        assert_eq!(s.solve(51), 51);
        assert_eq!(s.solve(30), 29);
    }
    #[test]
    fn test_2() {
        let s = Solution::new(&[1, 2, 3]);
        assert_eq!(s.solve(1), 3);
        assert_eq!(s.solve(2), 3);
        assert_eq!(s.solve(3), 3);
    }
    #[test]
    fn test_3() {
        let s = Solution::new(&[1, 2, 3]);
        assert_eq!(s.solve(4), 4);
        assert_eq!(s.solve(5), 5);
        assert_eq!(s.solve(6), 5);
    }
}
