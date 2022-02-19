use rand;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    // unimplemented!("Pick a private key greater than 1 and less than {}", p)
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate public key using prime numbers {} and {}, and private key {}",
    //     p,
    //     g,
    //     a
    // )
    modulo_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate secret key using prime number {}, public key {}, and private key {}",
    //     p,
    //     b_pub,
    //     a
    // )
    modulo_exp(b_pub, a, p)
}

pub fn modulo_exp(base: u64, exp: u64, modulor: u64) -> u64 {
    // unimplemented!(
    //     "Calculate {}^{} mod {}",
    //     base,
    //     exp,
    //     mod
    // )
    /*
    (m * n) % p = ((m % p) * (n % p)) % p

    8^5 % 6 = (8%6) * ((8^2) * (8^2)) % 6) = 2 * (((8*8)%6)^2)%6 = 2*4 % 6 = 2
    8^3 % 6 = ((8%6) * ((8%6) * (8%6)) % 6)) % 6
     */
    if modulor == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulor;
        }
        exp = exp >> 1;
        base = ((base % modulor) * (base %  modulor)) % modulor; // compute (base ^ 2) % modulor
    }
    result
}

