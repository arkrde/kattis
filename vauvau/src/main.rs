use std::io;
fn main() {
    let mut first_line = String::new();
    io::stdin()
        .read_line(&mut first_line)
        .expect("unable to read first line");
    if first_line.ends_with('\n') {
        first_line.pop();
    }
    let dog_values: Vec<i32> = first_line
        .split(' ')
        .filter(|w| w.len() > 0)
        .map(|w| w.trim().parse().unwrap())
        .collect();

    let mut second_line = String::new();
    io::stdin()
        .read_line(&mut second_line)
        .expect("unable to read second line");
    if second_line.ends_with('\n') {
        second_line.pop();
    }
    let arrivals: Vec<i32> = second_line
        .split(' ')
        .filter(|w| w.len() > 0)
        .map(|w| w.trim().parse().unwrap())
        .collect();
    for v in arrivals {
        println!(
            "{}",
            solution(
                dog_values[0],
                dog_values[1],
                dog_values[2],
                dog_values[3],
                v
            )
        );
    }
}

fn solution(a: i32, b: i32, c: i32, d: i32, q: i32) -> String {
    let first_dog_state = [a, b]
        .iter()
        .cycle()
        .scan(0, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .take_while(|v| v < &q)
        .count()
        % 2;
    let second_dog_state = [c, d]
        .iter()
        .cycle()
        .scan(0, |state, &x| {
            *state = *state + x;
            Some(*state)
        })
        .take_while(|v| v < &q)
        .count()
        % 2;

    if first_dog_state + second_dog_state == 0 {
        String::from("both")
    } else if first_dog_state + second_dog_state == 2 {
        String::from("none")
    } else {
        String::from("one")
    }
}

#[cfg(test)]
mod acm_icpc_intro_to_cp3_test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(solution(2, 2, 3, 3, 0), "both");
        assert_eq!(solution(2, 2, 3, 3, 1), "both");
        assert_eq!(solution(2, 2, 3, 3, 3), "one");
        assert_eq!(solution(2, 2, 3, 3, 4), "none");
    }
    #[test]
    fn test_2() {
        assert_eq!(solution(2, 3, 4, 5, 4), "one");
        assert_eq!(solution(2, 3, 4, 5, 9), "none");
        assert_eq!(solution(2, 3, 4, 5, 5), "none");
    }
}
