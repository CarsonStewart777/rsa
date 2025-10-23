mod primes;
use primes::SMALL_PRIMES;
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::OsRng;
use num_traits::{One, Zero};
use std::io::{self, Write};

pub struct PublicKey {
    pub e: BigUint,
    pub n: BigUint,
}

pub struct PrivateKey {
    pub d: BigUint,
    pub n: BigUint,
}



fn fails_trial_division (n: &BigUint) -> bool {

    for &prime in SMALL_PRIMES {
        // The prime from the list is a u32, so we just use it directly
        if n % prime == BigUint::zero() {
            return true;
        } 
        } return false;

    }



fn check_prime(n: &BigUint) -> bool {
    // Handle small numbers and even numbers
    if *n <= BigUint::from(3u32) {
        return *n == BigUint::from(2u32) || *n == BigUint::from(3u32);
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    if fails_trial_division(n) {
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
        if x == BigUint::one() || x == n_minus_1 {
            continue;
        } else {
            for _ in 0..(s - 1) {
                x = x.modpow(&BigUint::from(2u32), n);
                if x == BigUint::one() {
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
    let upper = (BigUint::from(1u64) << 1024) - BigUint::one();

    println!("Finding first prime (p)...");
    let mut num1;
    loop {
        num1 = rng.gen_biguint_range(&lower, &upper);
        if &num1 % 2u32 == BigUint::zero() {
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
        if &num2 % 2u32 == BigUint::zero() {
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


fn calculate_mod_and_totient(p: &BigUint, q: &BigUint) -> (BigUint, BigUint) {

    let n = p * q;
    let p_minus_1 = p - BigUint::one();
    let q_minus_1 = q - BigUint::one();
    let phi = &p_minus_1 * &q_minus_1;

    (n, phi) 
}


pub fn generate_public(e: &BigUint, n: &BigUint) -> PublicKey {

    PublicKey {
        e: e.clone(),
        n: n.clone(),
    }

}

pub fn generate_private(e: &BigUint, phi: &BigUint, n: &BigUint) -> PrivateKey {
    
let d = e.modinv(phi).expect("modinverse could not be computed");

PrivateKey {
    
    d: d,
    n: n.clone(),
    
}

}

fn encrypt(message: &BigUint, public_key: &PublicKey) -> BigUint {

    message.modpow(&public_key.e, &public_key.n)
}

fn decrypt(ciphertext: &BigUint, private_key: &PrivateKey) -> BigUint {
    ciphertext.modpow(&private_key.d, &private_key.n)
}


fn main() {
    let mut public_key_store: Option<PublicKey> = None;
    let mut private_key_store: Option<PrivateKey> = None;

    loop {
        println!("\n--- Rust RSA Menu ---");
        println!("1. Generate Keys");
        println!("2. Encrypt a Message");
        println!("3. Decrypt a Message");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let (p, q) = generate_numbers();
                let (n, phi) = calculate_mod_and_totient(&p, &q);
                let e = BigUint::from(65537u32);

                let public_key = generate_public(&e, &n);
                let private_key = generate_private(&e, &phi, &n);

                println!("\nPublic Key Generated:\n  e: {}\n  n: {}", public_key.e, public_key.n);
                println!("\nPrivate Key Generated:\n  d: {}\n  n: {}", private_key.d, private_key.n);
                
                public_key_store = Some(public_key);
                private_key_store = Some(private_key);
            }
            "2" => {
                match &public_key_store {
                    Some(public_key) => {
                        print!("Enter a secret message to encrypt: ");
                        io::stdout().flush().unwrap();
                        let mut message_string = String::new();
                        io::stdin()
                            .read_line(&mut message_string)
                            .expect("Failed to read line");
                        
                        let message_bytes = message_string.trim().as_bytes();
                        let message_number = BigUint::from_bytes_be(message_bytes);
                        
                        let ciphertext = encrypt(&message_number, public_key);
                        println!("\nEncrypted ciphertext (c):\n{}", ciphertext);
                    }
                    None => {
                        println!("\nError: Please generate keys (option 1) first.");
                    }
                }
            }
            "3" => {
                match &private_key_store {
                    Some(private_key) => {
                        print!("Enter the ciphertext number to decrypt: ");
                        io::stdout().flush().unwrap();
                        let mut ciphertext_string = String::new();
                        io::stdin()
                            .read_line(&mut ciphertext_string)
                            .expect("Failed to read line");

                        // Try to parse the string as a base-10 number
                        match BigUint::parse_bytes(ciphertext_string.trim().as_bytes(), 10) {
                            Some(ciphertext_number) => {
                                let decrypted_number = decrypt(&ciphertext_number, private_key);
                                let decrypted_bytes = decrypted_number.to_bytes_be();
                                match String::from_utf8(decrypted_bytes) {
                                    Ok(decrypted_string) => {
                                        println!("\nDecrypted message: {}", decrypted_string);
                                    }
                                    Err(_) => {
                                        println!("\nError: Decrypted data was not valid UTF-8.");
                                    }
                                }
                            }
                            None => {
                                println!("\nError: Invalid number. Please paste the exact ciphertext number.");
                            }
                        }
                    }
                    None => {
                        println!("\nError: Please generate keys (option 1) first.");
                    }
                }
            }
            "4" => {
                println!("Exiting.");
                break;
            }
            _ => {
                println!("\nInvalid choice. Please enter 1, 2, 3, or 4.");
            }
        }
    }
}

