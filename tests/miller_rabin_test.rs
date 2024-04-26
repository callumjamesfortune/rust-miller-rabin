use rust_miller_rabin::miller_rabin::miller_rabin;

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    #[test]
    fn test_negative_numbers() {

        let primes = vec![
            BigInt::from(-150),
            BigInt::from(-3),
            BigInt::from(0),
            BigInt::from(-1005050)
        ];
        for prime in primes {
            assert_eq!(miller_rabin(&prime), false);
        }
    }

    #[test]
    fn test_primes() {

        let primes = vec![
            BigInt::from(2),
            BigInt::from(3),
            BigInt::from(5),
            BigInt::from(7),
            BigInt::from(11),
            BigInt::from(17)
        ];
        for prime in primes {
            assert_eq!(miller_rabin(&prime), true);
        }
    }

    #[test]
    fn test_non_primes() {

        let non_primes = vec![
            BigInt::from(4),
            BigInt::from(6),
            BigInt::from(8),
            BigInt::from(9),
            BigInt::from(10),
            BigInt::from(12),
            BigInt::from(14),
            BigInt::from(15),
            BigInt::from(16),
            BigInt::from(18),
            BigInt::from(20),
        ];
        for non_prime in non_primes {
            assert_eq!(miller_rabin(&non_prime), false);
        }
    }

    #[test]
    fn test_large_primes() {

        let large_primes = vec![
            "115792089237316195423570985008687907853269984665640564039457584007913129639747",
            "1234567898765432123456789876543212345678987654321234567898765432123456789876543212345678987654321",
            "1900871281664822113126851573935413975471896789968515493666638539088027103802104498957191261465571"
        ];
        for prime_str in large_primes {
            let prime = BigInt::parse_bytes(prime_str.as_bytes(), 10).unwrap();
            assert_eq!(miller_rabin(&prime), true);
        }
    }

    #[test]
    fn test_large_non_primes() {

        let large_non_primes = vec![
            "115792089237316195423570985008687907853269984665640564039457584007913129639745",
            "957496696762772407665",
            "66895268355447102757",
            "973178024296493491",
            "49517601548356780882357658763458634586345782346585",
        ];
        for non_prime_str in large_non_primes {
            let non_prime = BigInt::parse_bytes(non_prime_str.as_bytes(), 10).unwrap();
            assert_eq!(miller_rabin(&non_prime), false);
        }
    }
}

