fn largest_prime_factor(mut n: usize) -> Option<usize> {
    for i in 2..n {
        while n % i == 0 {
            n /= i
        }
        if n == 1 {
            return Some(i);
        }
    }
    None
}

fn main() {
    let primes = primal::Primes::all().take(100_003);
    let n = primes.fold(0, |acc, b| (acc + b) % 1000);
    println!("{}", largest_prime_factor(n).unwrap());
}
