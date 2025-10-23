
mod primes;
use primes::SMALL_PRIMES;
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::OsRng;
use num_traits::One;

fn fails_trial_division (n: &BigUint) -> bool {

    for &prime in SMALL_PRIMES {

        if n % prime == BigUint::from(0u32) {
            return true;
        } 
        } return false;

    }



fn check_prime(n: &BigUint) -> bool {
    // Handle small numbers and even numbers
    if *n <= BigUint::from(3u32) {
        return *n == BigUint::from(2u32) || *n == BigUint::from(3u32);
    }
    if n % 2u32 == BigUint::from(0u32) {
        return false;
    }

    if fails_trial_division(n) == true {
        return false;
    }

    // get s and d
    let n_minus_1 = n - BigUint::one();
    let s = n_minus_1.trailing_zeros().unwrap_or(0);
    let d = &n_minus_1 >> s;
    let mut rng = OsRng;

    // generate a 64 times
    for _ in 0..64 {
        let a = rng.gen_biguint_range(&BigUint::from(2u32), &n_minus_1);

        // find x using our values
        let mut x = a.modpow(&d, n);
        if x == BigUint::from(1u32) || x == n_minus_1 {
            continue;
        } else {
            for _ in 0..(s - 1) {
                x = x.modpow(&BigUint::from(2u32), n);
                if x == BigUint::from(1u32) {
                    return false;
                } else if x == n_minus_1 {
                    break;
                }
            }
            if x != n_minus_1 {
                return false;
            }
        }
    }
    return true;
}

fn generate_numbers() -> (BigUint, BigUint) {
    let mut rng = OsRng;

    // set lower and upper bounds
    let lower = BigUint::from(1u64) << 1023;
    let upper = (BigUint::from(1u64) << 1024) - 1u32;

    println!("Finding first prime (p)...");
    let mut num1;
    loop {
        num1 = rng.gen_biguint_range(&lower, &upper);
        if &num1 % 2u32 == BigUint::from(0u32) {
            num1 += BigUint::one();
            if num1 >= upper {
                continue;
            }
        }
        
        if check_prime(&num1) {
            break;
        }
    }
    println!("Found p!");

    println!("Finding second prime (q)...");
    let mut num2;
    loop {
        num2 = rng.gen_biguint_range(&lower, &upper);
        if &num2 % 2u32 == BigUint::from(0u32) {
            num2 += BigUint::one();
            if num2 >= upper {
                continue;
            }
        }
        
        if num1 != num2 && check_prime(&num2) {
            break;
        }
    }
    println!("Found q!");

    (num1, num2)
}


fn generate_keys(p: &BigUint, q: &BigUint) -> (BigUint, BigUint) {


    let n = p * q;
    let phi = (p - BigUint::from(1u32)) * (q - BigUint::from(1u32));

    (n, phi) 



}
fn main() {
    let (num1, num2) = generate_numbers();
    let res1 = check_prime(&num1);
    println!("Testing num1: {}", num1);
    println!("Result: {}", if res1 { "Probably Prime" } else { "Composite!" });
    let res2 = check_prime(&num2);
    println!("\nTesting num2: {}", num2);
    println!("Result: {}", if res2 { "Probably Prime" } else { "Composite!" });
    let (n, phi) = generate_keys(&num1, &num2);
    println!("\nThe Product of these primes is: {}", n);
    println!("\n The Totien (phi) is: {}", phi)
}