pub mod miller_rabin;

// ! This is a library for performing primality testing using the Miller-Rabin algorithm.
// !
// ! Heavily based on the following repository:
// !
// ! https://github.com/jsanders/rust-rsa
// !
// ! # Usage
// !
// ! To use this library, add it as a dependency in your `Cargo.toml`:
// !
// ! ```toml
// ! [dependencies]
// ! rust_miller_rabin = "0.1.0"
// ! ```
// !
// ! Then, in your Rust code, you can use it as follows:
// !
// ! ```rust
// ! use rust_miller_rabin::miller_rabin::miller_rabin;
// ! use num_bigint::BigInt;
// !
// ! fn main() {
// !     let num = BigInt::from(1234567890);
// !     if is_prime(&num) {
// !         println!("{} is probably prime", num);
// !     } else {
// !         println!("{} is composite", num);
// !     }
// ! }
// ! '''