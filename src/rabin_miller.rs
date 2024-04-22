use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, One};
use num_integer::Integer;
use rand::{Rng, thread_rng};
use num_traits::ToPrimitive;

pub fn rabin_miller(candidate: &BigUint) -> bool {
    if candidate.is_even() {
        return false; // Even numbers are not prime
    }
    let zero = BigUint::zero();
    let one = BigUint::one();
    let two = &one + &one;
  
    if candidate == &two   { return true }
    if candidate.is_even() { return false }
  
    let (s, d) = rewrite(&(candidate - &one));
  
    let mut k = 0;
    while k < 128 {
        let mut rng = thread_rng(); 
        let basis: BigUint = if *candidate > BigUint::from(1u8) {
            let candidate_u64 = candidate.to_u64().unwrap();
            rng.gen_range(two.to_u64().unwrap()..(candidate_u64 - 1)).to_biguint().unwrap()
        } else {
            rng.gen_range(two.to_u64().unwrap()..1).to_biguint().unwrap()
        };      
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

fn rewrite(candidate: &BigUint) -> (BigUint, BigUint) {
    let zero = BigUint::zero();
    let one = BigUint::one();
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



fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exponent, modulus)
}
