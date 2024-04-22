use num_bigint::BigInt;
use rust_miller_rabin::rabin_miller::rabin_miller;

fn main() {

    let candidate: BigInt = "1900871281664822113126851573935413975471896789968515493666638539088027103802104498957191261465571".parse().unwrap();

    let prime = rabin_miller(&candidate);

    if prime == false {
        println!("{} is not prime", candidate);
    } else {
        println!("{} is prime", candidate);
    }

}
