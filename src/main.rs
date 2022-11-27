
// This is my code that I (J. Romano) originally placed in:
//    https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test#Rust
//
// I ported the code to Rust from the pseudocode on this Wikipedia page:
//    https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Miller%E2%80%93Rabin_test


/* Add these lines to the [dependencies] section of your Cargo.toml file:
num = "0.4.0"
rand = "0.8.5"
*/

use num::bigint::BigInt;
use num::bigint::ToBigInt;


// The modular_exponentiation() function takes three identical types
// (which get cast to BigInt), and returns a BigInt:
fn modular_exponentiation<T: ToBigInt>(n: &T, e: &T, m: &T) -> BigInt {
    // Convert n, e, and m to BigInt:
    let n = n.to_bigint().unwrap();
    let e = e.to_bigint().unwrap();
    let m = m.to_bigint().unwrap();

    // Sanity check:  Verify that the exponent is not negative:
    assert!(e >= Zero::zero());

    use num::traits::{Zero, One};

    // As most modular exponentiations do, return 1 if the exponent is 0:
    if e == Zero::zero() {
        return One::one()
    }

    // Now do the modular exponentiation algorithm:
    let mut result: BigInt = One::one();
    let mut base = n % &m;
    let mut exp = e;

    loop {  // Loop until we can return our result.
        if &exp % 2 == One::one() {
            result *= &base;
            result %= &m;
        }

        if exp == One::one() {
            return result
        }

        exp /= 2;
        base = &base * &base;
        base %= &m;
    }
}


// is_prime() checks the passed-in number against many known small primes.
// If that doesn't determine if the number is prime or not, then the number
// will be passed to the is_rabin_miller_prime() function:
fn is_prime<T: ToBigInt>(n: &T) -> bool {
    let n = n.to_bigint().unwrap();
    if n < 2.to_bigint().unwrap() {
        return false
    }

    let small_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43,
                            47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
                            103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
                            157, 163, 167, 173, 179, 181, 191, 193, 197, 199,
                            211, 223, 227, 229, 233, 239, 241, 251, 257, 263,
                            269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
                            331, 337, 347, 349, 353, 359, 367, 373, 379, 383,
                            389, 397, 401, 409, 419, 421, 431, 433, 439, 443,
                            449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
                            509, 521, 523, 541, 547, 557, 563, 569, 571, 577,
                            587, 593, 599, 601, 607, 613, 617, 619, 631, 641,
                            643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
                            709, 719, 727, 733, 739, 743, 751, 757, 761, 769,
                            773, 787, 797, 809, 811, 821, 823, 827, 829, 839,
                            853, 857, 859, 863, 877, 881, 883, 887, 907, 911,
                            919, 929, 937, 941, 947, 953, 967, 971, 977, 983,
                            991, 997, 1009, 1013];

    use num::traits::Zero;  // for Zero::zero()

    // Check to see if our number is a small prime (which means it's prime),
    // or a multiple of a small prime (which means it's not prime):
    for sp in small_primes {
        let sp = sp.to_bigint().unwrap();

        if n == sp {
            return true
        } else if &n % sp == Zero::zero() {
            return false
        }
    }

    is_rabin_miller_prime(&n, None)
}


// Note:  "use bigint::RandBigInt;"  (which is needed for gen_bigint_range())
//        fails to work in the Rust playground ( https://play.rust-lang.org ).
//        Therefore, I'll create my own here:
fn get_random_bigint(low: &BigInt, high: &BigInt) -> BigInt {
    if low == high {  // base case
        return low.clone()
    }

    let middle = (low + high) / 2.to_bigint().unwrap();

    let go_low: bool = rand::random();

    return if go_low {
        get_random_bigint(low, &middle)
    } else {
        get_random_bigint(&middle, high)
    }
}


// This is the Rabin-Miller primality test ported from:
//    https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Miller%E2%80%93Rabin_test
// Pseudocode:
/*
Input #1: n > 2, an odd integer to be tested for primality
Input #2: k, the number of rounds of testing to perform
Output: “composite” if n is found to be composite, “probably prime” otherwise

let s > 0 and d odd > 0 such that n − 1 = 2sd  # by factoring out powers of 2 from n − 1
repeat k times:
    a ← random(2, n − 2)  # n is always a probable prime to base 1 and n − 1
    x ← ad mod n
    repeat s times:
        y ← x2 mod n
        if y = 1 and x ≠ 1 and x ≠ n − 1 then  # nontrivial square root of 1 modulo n
            return “composite”
        x ← y
    if y ≠ 1 then
        return “composite”
return “probably prime”
*/
// k is the number of times for testing (pass in None to use 10 (the default)).
fn is_rabin_miller_prime<T: ToBigInt>(n: &T, k: Option<usize>) -> bool {
    let n = n.to_bigint().unwrap();
    let k = k.unwrap_or(10);  // number of times for testing (defaults to 10)

    use num::traits::{Zero, One};  // for Zero::zero() and One::one()
    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();
    let two: BigInt = 2.to_bigint().unwrap();

    // The call to is_prime() should have already checked this,
    // but check for two, less than two, and multiples of two:
    if n <= one {
        return false
    } else if n == two {
        return true  // 2 is prime
    } else if &n % &two == Zero::zero() {
        return false  // even number (that's not 2) is not prime
    }

    let mut t: BigInt = zero.clone();
    let n_minus_one: BigInt = &n - &one;
    let mut s = n_minus_one.clone();
    while &s % &two == one {
        s /= &two;
        t += &one;
    }

    // Try k times to test if our number is non-prime:
    'outer: for _ in 0..k {
        let a = get_random_bigint(&two, &n_minus_one);
        let mut v = modular_exponentiation(&a, &s, &n);
        if v == one {
            continue 'outer;
        }
        let mut i: BigInt = zero.clone();
        while &i < &t {
            v = (&v * &v) % &n;
            if v == n_minus_one {
                continue 'outer;
            }
            i += &one;
        }
        return false
    }
    // If we get here, then we have a degree of certainty
    // that n really is a prime number, so return true:
    true
}


fn test() {
    println!();

    let n = 1234687;
    let result = is_prime(&n);
    println!("Q: Is {n} prime?  A: {result}");
    println!("(The answer should be true.)");
    println!();

    let n = 1234689;
    let result = is_prime(&n);
    println!("Q: Is {n} prime?  A: {result}");
    println!("(The answer should be false.)");
    println!();

    let n = BigInt::parse_bytes("123123423463".as_bytes(), 10).unwrap();
    let result = is_prime(&n);
    println!("Q: Is {n} prime?  A: {result}");
    println!("(The answer should be true.)");
    println!();

    let n = BigInt::parse_bytes("123123423465".as_bytes(), 10).unwrap();
    let result = is_prime(&n);
    println!("Q: Is {n} prime?  A: {result}");
    println!("(The answer should be false.)");
    println!();

    let n = BigInt::parse_bytes("123123423467".as_bytes(), 10).unwrap();
    let result = is_prime(&n);
    println!("Q: Is {n} prime?  A: {result}");
    println!("(The answer should be false.)");
    println!();

    let n = BigInt::parse_bytes("123123423469".as_bytes(), 10).unwrap();
    let result = is_prime(&n);
    println!("Q: Is {n} prime?  A: {result}");
    println!("(The answer should be true.)");
    println!();
}


// Returns the help text suitable for printing when the
// user specifies the --help switch.
fn help_text() -> String {
    format!("
Program:  miller-rabin-primality-test
          An implementation of the Miller-Rabin primality test written in Rust.
Usage:  miller-rabin-primality-test [--help] [--test] [NUM [NUM [...] ]
Example usages:
   miller-rabin-primality-test   # (interactive mode)
   miller-rabin-primality-test 5 7 9 11
   miller-rabin-primality-test --test
Options:
   -h, --help
      Shows this help text and exits.
   --test
      Runs tests and exits.
Author:  Jean-Luc Romano
e-mail:  {username}@{domain}.{suffix}

About:  This is code I originally posted to Rosetta Code here:
        https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test#Rust
", username = "jl_post", domain = "hotmail", suffix = "com")
}


fn main() {
    // Get command-line arguments:
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        // Parse command-line arguments:
        let mut still_looking_for_options = true;
        for arg in &args {
            if still_looking_for_options && arg == "--" {  // (The "--" option signifies the end of the options.)
                still_looking_for_options = false;
            } else if still_looking_for_options && (arg == "-h" || arg == "--help") {
                println!("{}", help_text());
                return ()
            } else if still_looking_for_options && arg == "--test" {
                test();
                return ()
            } else if still_looking_for_options && arg.starts_with("--") {
                println!("Error:  Invalid option:  {arg}");
                std::process::exit(1)
            } else {
                // We (probably) have a number to test.
                match BigInt::parse_bytes(arg.as_bytes(), 10) {
                    Some(n) => {
                        // We found a valid number.
                        let result = is_prime(&n);
                        println!("Q: Is {n} prime?  A: {result}");
                    }
                    _ => {
                        // We found an invalid number.
                        println!("Error:  Invalid number:  {arg}");
                        std::process::exit(1)
                    }
                }
            }
        }
    // (End of parsing command-line arguments.)
    } else {
        // No arguments were given, so do interactive mode.
        loop {
            // Prompt the user for input:
            print!("\nEnter a number to test:  ");
            use std::io::Write;  // (for using .flush() on std::io::stdout() )
            std::io::stdout().flush().expect("Error on flush().");

            // Get the user's input:
            let mut input_string = String::new();
            std::io::stdin().read_line(&mut input_string).expect("Failed to read input.");
            let input_string = input_string.trim();

            match BigInt::parse_bytes(input_string.as_bytes(), 10) {
                Some(n) => {
                    // We found a valid number.
                    let result = is_prime(&n);
                    println!("Q: Is {n} prime?  A: {result}");
                }
                _ => {
                    // We found an invalid number.
                    println!("\n\"{input_string}\" is not a valid number.\nExiting...\n");
                    return ()
                }
            }
        }
        // (End of interactive mode.)
    }
}


