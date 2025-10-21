use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

fn generate_numbers() -> (BigUint, BigUint) {

    let mut rng = thread_rng();
    
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

fn check_prime(num1: &BigUint, num2: &BigUint) {

    println!("num1 : {}", num1);
    println!("num2 : {}", num2);

    
}

fn main() {
let (num1, num2) = generate_numbers();
check_prime(&num1, &num2);
}
