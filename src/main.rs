use num_bigint::{BigUint, RandBigInt};
use rand::rngs::OsRng;
use num_traits::One;

fn generate_numbers() -> (BigUint, BigUint) {

    let mut rng = OsRng;

    // set lower and upper bounds

    let  lower = BigUint::from(1u64) << 1023;
    let upper = (BigUint::from(1u64) << 1024) - 1u32;

    // generate odd num1

    let mut num1 = rng.gen_biguint_range(&lower, &upper);
    while &num1 % 2u32 == BigUint::from(0u32) {
        num1 = rng.gen_biguint_range(&lower, &upper);
    }

    // generate odd num2

    let mut num2 = rng.gen_biguint_range(&lower, &upper);
    while &num2 % 2u32 == BigUint::from(0u32) {
        num2 = rng.gen_biguint_range(&lower, &upper);
    }


    (num1, num2)






}

fn check_prime(n: &BigUint) -> bool {

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
            continue
        } else {
            for _ in 0..(s-1) {
                x = x.modpow(&BigUint::from(2u32), n);
                if x == BigUint::from(1u32) {
                    return false;
                }
                else if x == n_minus_1 {
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

fn main() {
    let (num1, num2) = generate_numbers();
    let _ = check_prime(&num1);
    let _ = check_prime(&num2);
}
