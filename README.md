# <img src="https://github.com/callum-fortune/rust-miller-rabin/assets/63158857/19ebacb0-75c5-4a59-9549-590513dec5e9" width="100">
# 🚀  Rust Miller-Rabin with bigInt capability
A complete Rust implementation of the Miller Rabin primality test algorithm, based heavily on (https://github.com/jsanders/rust-rsa).

### Notes

- The Miller Rabin algorithm, whilst incredibly accurate has been known to produce false positives in some cases. This is because the algorithm is officially classed as a probabilistic primality test. Proceed to use this project with that knowledge. Read more here...

  https://core.ac.uk/download/pdf/197479038.pdf

- I am yet to be made aware of the higher limit of this project, I have tested the code with prime numbers more than 300 characters in length and as little as one, without fail.
- I created this project on my journey to implenting a basic Rust RSA implementation. This is a common use case for the Miller-Rabin test and there is a good chance that you are in the same process. Be warned that any implementations of your own that use this code are done at your own risk.

### Usage

- I have packaged this project as a Rust library however I have included a main.rs file to show the code in a working state. This can be found in the src folder or just ran with:
  
  ```
  cargo build
  cargo run
  ```
- I have included tests for the project which will test small primes, large primes, small non-primes and large non-primes. Run these with:

  ```
  cargo test
  ```
  ^Not to insult your intelligence
