use num_bigint::BigUint;
use rust_miller_rabin::rabin_miller::rabin_miller;

fn main() {
    // Create a BigUint to test
    let candidate: BigUint = "10757746757434227797".parse().unwrap();

    let prime = rabin_miller(&candidate);

    if prime == false {
        println!("{} is not prime", candidate);
    } else {
        println!("{} is prime", candidate);
    }

}
