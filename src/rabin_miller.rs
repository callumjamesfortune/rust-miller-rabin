use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero, One};
use num_integer::Integer;
use rand::thread_rng;

pub fn rabin_miller(candidate: &BigInt) -> bool {
    if candidate.is_even() {
        return *candidate == BigInt::from(2); // 2 is prime
    }

    if candidate.is_zero() || *candidate == BigInt::from(1) {
        return false; // 0 and 1 are not prime
    }

    if candidate.to_string().len() == 1 {
        // Handling single-digit numbers
        let small_primes: [usize; 4] = [2, 3, 5, 7];
        return small_primes.contains(&candidate.to_string().parse().unwrap());
    }

    let zero = BigInt::zero();
    let one = BigInt::one();
    let two = &one + &one;

    if *candidate == two {
        return true;
    }

    let (shift, odd_factor) = decompose(&(candidate - &one));

    let mut iterations = 0;
    let mut rng = thread_rng();

    while iterations < 128 {
        let mut basis: BigInt = rng.gen_bigint(candidate.bits());
        while basis <= BigInt::one() || &basis >= &(candidate - &one) {
            basis = rng.gen_bigint(candidate.bits());
        }
        let mut witness = mod_exp(&basis, &odd_factor, candidate);
        if witness != one && witness != (candidate - &one) {
            let mut exponent = zero.clone();
            loop {
                witness = mod_exp(&witness, &two, candidate);
                if witness == (candidate - &one) {
                    break;
                } else if witness == one || exponent == (shift.clone() - &one) {
                    return false;
                }
                exponent += &one;
            }
        }
        iterations += 2;
    }
    true
}

fn decompose(candidate: &BigInt) -> (BigInt, BigInt) {
    let zero = BigInt::zero();
    let one = BigInt::one();
    let two = &one + &one;

    let mut shift = zero.clone();
    let mut odd_factor = candidate.clone();
    while odd_factor.is_even() {
        odd_factor /= &two;
        shift += &one;
    }
    if odd_factor == zero {
        odd_factor = one.clone();
    }
    (shift, odd_factor)
}

fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    base.modpow(exponent, modulus)
}
