use std::f64::consts::E;
use std::io;
fn main() {
    let mut new_line = String::new();
    io::stdin()
        .read_line(&mut new_line)
        .expect("Could not read new line");
    if new_line.ends_with("\n") {
        new_line.pop();
    }
    let n: f64 = new_line.trim().parse().unwrap();
    println!("{:.6}", solution(n));
}

fn solution(n: f64) -> f64 {
    E.powf(n.ln() / n)
}

#[cfg(test)]
mod acm_icpc_intro_to_cp3_test {
    use super::*;
    #[test]
    fn test() {
        //assert!( < f64::EPSILON);
        assert_eq!(format!("{:.6}", solution(2.0_f64)), "1.414214");
        assert_eq!(format!("{:.6}", solution(1.0_f64)), "1.000000");
        assert_eq!(format!("{:.6}", solution(1.5_f64)), "1.310371");
    }
}
