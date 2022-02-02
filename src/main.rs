struct PrimeList {
    highest_prime: i32,
    highest_checked: i32,
    prime_list: Vec<i32>,
}

impl PrimeList {

    // finds the next highest prime number
    fn next_prime(&mut self) -> i32 {
        loop {
            self.highest_checked += 1;
            if is_prime(self.highest_checked) {
                self.prime_list.push(self.highest_checked);
                self.highest_prime = self.highest_checked;
                return self.highest_prime
            }
        }
    }

    // checks for primes up to the square root of target number "n"
    fn check_for(&mut self, n: i32) {
    }
}

// checks if number "n" is prime
fn is_prime(n: i32) -> bool {
    let upper_bound = (n as f64).powf(0.5) + 1.0;

    for i in 2..(upper_bound as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// returns a vector of the prime factorization of number "n"
fn prime_factorize(n: i32, list: PrimeList) -> Vec<i32> {
    if list.highest_checked >
}

fn main() {
    let primes = (2..=10000).filter(|n| is_prime(*n)).collect::<Vec<i32>>();
    println!("{:#?}", primes);
}
