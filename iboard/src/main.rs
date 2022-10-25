use std::io;
fn main() {
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");
        if line.ends_with('\n') {
            line.pop();
        }
        if line.len() == 0 {
            break;
        }
        println!("{}", solution(&line));
    }
}

fn solution_impl(n: u8) -> (usize, usize) {
    let mut one_count: usize = 0;
    let mut n = n;
    while n != 0 {
        if n & 1 == 1 {
            one_count += 1;
        }
        n = n >> 1;
    }
    (7 - one_count, one_count)
}

fn solution(seq: &str) -> String {
    let seq: Vec<u8> = seq.as_bytes().to_owned();
    let results = seq.into_iter().fold((0, 0), |acc, v| {
        let count = solution_impl(v);
        (acc.0 + count.0, acc.1 + count.1)
    });
    if results.0 & 1 == 0 && results.1 & 1 == 0 {
        String::from("free")
    } else {
        String::from("trapped")
    }
}

#[cfg(test)]
mod acm_icpc_intro_to_cp3_test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(solution_impl(105), (3, 4));
    }
    #[test]
    fn test_2() {
        assert_eq!(solution("Keep up the good work."), "trapped");
        assert_eq!(solution("iBoard Rules!!"), "free");
        assert_eq!(solution("qwerty"), "trapped");
    }
}
