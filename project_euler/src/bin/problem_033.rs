// Solution to problem 033
// https://projecteuler.net/problem=33

project_euler::solution!(33);

pub fn solve_problem_33() -> Option<u64> {
    None
}

struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Fraction {
    fn new(numerator: u64, denominator: u64) -> Self {
        Self { numerator, denominator }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_official_answer() {
        assert_eq!(solve_problem_33(), None);
    }
}
