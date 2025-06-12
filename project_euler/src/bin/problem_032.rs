// Solution to problem 032
// https://projecteuler.net/problem=32

project_euler::solution!(32);

use std::collections::HashSet;

pub fn solve_problem_32() -> Option<u64> {
    // initialise heap helper for our digit permutations
    let digits = HeapHelper::new();
    // iterate over all digit permutations
    let output: u64 = digits.into_iter()
        // splice into the 4 identity cases
        .flat_map(|perm| get_identity_cases(perm))
        // evaluate and filter out any cases that are not valid
        .filter_map(|case| evaluate_identity_case(case))
        // collect into a hashset to remove any duplicated values
        .collect::<HashSet<u64>>()
        // iterate and sum all values in the hashset
        .iter()
        .sum();
    Some(output)
}

const ALL_DIGITS: [u64; 9] = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9
];

struct HeapHelper {
    a: [u64; 9],
    c: [usize; 9],
    i: usize,
    first: bool
}

impl HeapHelper {
    fn new() -> Self {
        Self {
            a: ALL_DIGITS,
            c: [0; 9],
            i: 0,
            first: true
        }
    }
}

impl Iterator for HeapHelper {
    type Item = [u64; 9];

    fn next(&mut self) -> Option<Self::Item> {
        // initial case
        if self.first {
            self.first = false;
            return Some(self.a)
        }
        while self.i < 9 {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.a.swap(0, self.i);
                } else {
                    self.a.swap(self.c[self.i], self.i);
                }
                self.c[self.i] += 1;
                self.i = 0;
                return Some(self.a) 
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        None
    }
}

enum IdentityCase {
    /// 1 * 1_000 = 1_000
    OneByFourMakesFour([u64;1], [u64; 4], [u64; 4]),
    /// 10 * 100 = 1_000
    TwoByThreeMakesFour([u64; 2], [u64; 3], [u64; 4]),
    /// 100 * 10 = 1_000
    ThreeByTwoMakesFour([u64; 3], [u64; 2], [u64; 4]),
    /// 1_000 * 1 = 1_000
    FourByOneMakesFour([u64; 4], [u64; 1], [u64; 4]),
}

fn get_identity_cases(arr: [u64; 9]) -> [IdentityCase; 4] {
    use IdentityCase::*;
    [
        OneByFourMakesFour(
            [arr[0]], 
            [
                arr[1],
                arr[2],
                arr[3],
                arr[4],
            ],
            [
                arr[5],
                arr[6],
                arr[7],
                arr[8]
            ]
        ),
        TwoByThreeMakesFour(
            [
                arr[0],
                arr[1],
            ],
            [
                arr[2],
                arr[3],
                arr[4],
            ],
            [
                arr[5],
                arr[6],
                arr[7],
                arr[8]
            ]
        ),
        ThreeByTwoMakesFour(
            [
                arr[0],
                arr[1],
                arr[2]
            ],
            [
                arr[3],
                arr[4]
            ],
            [
                arr[5],
                arr[6],
                arr[7],
                arr[8]
            ]
        ),
        FourByOneMakesFour(
            [
                arr[0],
                arr[1],
                arr[2],
                arr[3]
            ],
            [
                arr[4]
            ],
            [
                arr[5],
                arr[6],
                arr[7],
                arr[8]
            ]
        )
    ]
}

fn evaluate_identity_case(case: IdentityCase) -> Option<u64> {
    use IdentityCase::*;

    // extract our numbers
    let (a, b, c) = match case {
        OneByFourMakesFour(a, b, c) => {
            (digits_to_num(&a), digits_to_num(&b), digits_to_num(&c))
        },
        TwoByThreeMakesFour(a, b, c) => {
            (digits_to_num(&a), digits_to_num(&b), digits_to_num(&c))
        },
        ThreeByTwoMakesFour(a, b, c) => {
            (digits_to_num(&a), digits_to_num(&b), digits_to_num(&c))
        },
        FourByOneMakesFour(a, b, c) => {
            (digits_to_num(&a), digits_to_num(&b), digits_to_num(&c))
        },
    };

    // test and return
    if a * b == c {
        Some(c)
    } else {
        None
    }
}

fn digits_to_num(slice: &[u64]) -> u64 {
    slice.iter()
        .rev()
        .enumerate()
        .map(|(pow, &digit)| digit * (0..pow).fold(1, |acc, _| acc * 10))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_official_answer() {
        assert_eq!(solve_problem_32(), Some(45228));
    }
}
