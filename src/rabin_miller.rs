use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero, One};
use num_integer::Integer;
use rand::thread_rng;

pub fn rabin_miller(candidate: &BigInt) -> bool {
    if candidate.is_even() {
        return false; // Even numbers are not prime
    }
    let zero = BigInt::zero();
    let one = BigInt::one();
    let two = &one + &one;
  
    if candidate == &two   { return true }
    if candidate.is_even() { return false }
  
    let (s, d) = rewrite(&(candidate - &one));

    let mut k = 0;
    while k < 128 {
        let mut rng = thread_rng(); 
        let mut basis: BigInt = rng.gen_bigint(candidate.bits());
        while basis <= BigInt::one() || basis >= candidate - &one {
            basis = rng.gen_bigint(candidate.bits());
        }
        let mut v = mod_exp(&basis, &d, candidate);
        if v != one && v != (candidate - &one) {
            let mut i = zero.clone();
            loop {
                v = mod_exp(&v, &two, candidate);
                if v == (candidate - &one) {
                    break;
                } else if v == one || i == (s.clone() - &one) {
                    return false
                }
                i += &one;
            }
        }
        k += 2;
    }
    true
}

fn rewrite(candidate: &BigInt) -> (BigInt, BigInt) {
    let zero = BigInt::zero();
    let one = BigInt::one();
    let two = &one + &one;
    
    let mut s = zero.clone();
    let mut d = candidate.clone();
    while d.is_even() {
        d /= &two;
        s += &one;
    }
    if d == zero {
        d = one.clone();
    }
    (s, d)
}

fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    base.modpow(exponent, modulus)
}

