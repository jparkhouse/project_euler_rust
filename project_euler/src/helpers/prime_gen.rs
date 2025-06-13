pub struct PrimeGenerator {
    memory: Vec<bool>
}

impl PrimeGenerator {
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }
    pub fn with_capacity(n: usize) -> Self;
    pub fn get_nth_prime(&mut self, n: usize) -> usize;
    pub fn get_last_prime_before(&mut self, n: usize) -> usize;
    pub fn get_all_primes_up_to(&mut self, n: usize) -> Vec<usize>;

    fn extend_to(&mut self, n: usize);
    fn convert_to_primes(&self) -> Vec<usize>;
}