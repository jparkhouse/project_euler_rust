use std::cmp::max;

/// PrimeGenerator stores a cache of ya boii's sieve, and exposes a number of
/// methods for interacting with the primes it calculates. Assuming it is used mutably, 
/// it will auto-extend the cache.
pub struct PrimeGenerator {
    memory: Vec<bool>
}

impl PrimeGenerator {
    /// Returns a new instance of PrimeGenerator, populated up to 2
    pub fn new() -> Self {
        Self { memory: vec![false, false, true] }
    }

    pub fn with_capacity(n: usize) -> Self {
        let mut new = Self::new();
        new.extend_to(n);
        new
    }

    pub fn get_nth_prime(&mut self, n: usize) -> usize {
        if n == 0 {
            return 2
        }
        loop {
            let nth_prime = self.convert_to_prime_iter()
                // since we have already consumed the n = 0 case, this is safe
                .skip(n - 1)
                .next();

            match nth_prime {
                Some(prime) => return prime,
                None => {
                    // extend the sieve and try again
                    self.extend_to(self.approximate_prime_size(n));
                }
            }
        }
        
    }

    pub fn get_last_prime_before(&mut self, n: usize) -> usize {
        self.extend_to(n + 1);
        self.convert_to_prime_iter().fold(0, |acc, next| {
            if next < n {
                next
            } else {
                acc
            }
        })
    }

    pub fn get_all_primes_up_to(&mut self, n: usize) -> Vec<usize> {
        self.extend_to(n + 1);
        self.convert_to_prime_iter()
            .filter(|&prime| prime < n)
            .collect()
    }

    pub fn get_prime_factors_of(&mut self, n: usize) -> Vec<usize> {
        // we can return early if we already know n is prime
        // this will also ensure we have the primes for the prime buffer
        if self.is_prime(n) {
            return vec![n]
        }
        let prime_buffer = self.get_all_primes_up_to(n);
        let mut n_fac = n;
        let mut output = Vec::with_capacity(8);
        while n_fac > 1 {
            let next_factor = prime_buffer.iter()
                .find(|&&prime| n_fac % prime == 0)
                .expect("we have checked that it is not prime, therefore must have some prime factor")
                .to_owned();
            output.push(next_factor);
            n_fac /= next_factor;
        }
        output
    }

    pub fn is_prime(&mut self, n: usize) -> bool {
        self.extend_to(n + 1);
        self.memory[n]
    }

    pub fn convert_to_prime_iter(&self) -> impl Iterator<Item = usize> {
        self.memory.iter()
            .enumerate()
            .filter_map(
                |(ind, is_prime)| {
                    match is_prime {
                        true => Some(ind),
                        false => None
                    }
            })
    }

    fn extend_to(&mut self, n: usize) {
        // establish how much extension we need, and return early if we
        // are already at that size or larger
        let gap = match n.checked_sub(self.memory.len()) {
            Some(val) => val,
            None => return
        };

        // extend the memory with true values
        self.memory.extend_from_slice(vec![true; gap].as_slice());
        // iterate over and turn off all non-primes in the new memory
        let max = self.memory.len();
        for ind in 0 .. max {
            if self.memory[ind] { // true when prime
                // start from the second multiple of that prime, and run in multiples until the end
                for non_prime_ind in (2*ind .. max).step_by(ind) {
                    // multiple of ind must be non-prime
                    self.memory[non_prime_ind] = false
                }
            }
        }
    }

    /// Uses the prime number theorem to suggest an approximate size
    /// to extend to for the nth prime. If this is insufficient, will do a 20% extension.
    fn approximate_prime_size(&self, n: usize) -> usize {
        let n = n as f64;
        let log_n = n.ln();
        let log_log_n = log_n.ln();
        let pnt_guess = (1.05 * n * (log_n + log_log_n)) as usize;
        let standard_extension = (n * 1.2) as usize;
        let mimimum_sensible_extension: usize = 50;
        if pnt_guess <= mimimum_sensible_extension && standard_extension <= mimimum_sensible_extension {
            return mimimum_sensible_extension
        } else {
            return max(standard_extension, pnt_guess)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::prime_gen::PrimeGenerator;

    #[test]
    fn can_correctly_identify_prime_numbers() {
        // arrange
        let mut prime_gen = PrimeGenerator::new();

        // act
        let two_is_prime = prime_gen.is_prime(2);
        let five_is_prime = prime_gen.is_prime(5);
        let thirty_is_not_prime = !prime_gen.is_prime(30);
        let sixty_one_is_prime = prime_gen.is_prime(61);

        // assert
        assert!(two_is_prime);
        assert!(five_is_prime);
        assert!(thirty_is_not_prime);
        assert!(sixty_one_is_prime);
    }

    #[test]
    fn finds_prime_factors_of_28() {
        // arrange
        let mut prime_gen = PrimeGenerator::new();

        // act
        let prime_factors = prime_gen.get_prime_factors_of(28);
        // expected to be [2, 2, 7]

        // assert
        assert_eq!(prime_factors.len(), 3);
        assert_eq!(prime_factors[0], 2);
        assert_eq!(prime_factors[1], 2);
        assert_eq!(prime_factors[2], 7);
    }

    #[test]
    fn finds_prime_factors_of_prime() {
        // arrange
        let mut prime_gen = PrimeGenerator::new();
        
        // act
        let prime_factors = prime_gen.get_prime_factors_of(17);
        
        // assert
        assert_eq!(prime_factors.as_slice(), [17])
    }
}