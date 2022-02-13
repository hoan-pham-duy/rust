pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    //Find the nth prime number
    //1. Create a vector of primes
    //2. Return the nth prime number
    let mut primes = vec![2];
    let mut num = 3;
    while primes.len() <= n as usize {
        let mut is_prime = true;
        for prime in &primes {
            if num % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
        num += 2;
    }
    primes[n as usize]
}
