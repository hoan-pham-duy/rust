pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {}", n);
    //Primes factors of number
    let mut num = n;
    let mut factors = Vec::new();
    let mut factor = 2;
    while num > 1 {
        if num % factor == 0 {
            factors.push(factor);
            num /= factor;
        } else {
            factor += 1;
        }
    }
    factors
}
