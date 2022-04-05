// Bring the standard library trait "FromStr" into scope
// Never explicitly used, but required for the "from_str" call from the u64 type
use std::str::FromStr;

// Several useful functions and types for interacting with the execution
// environment, including "args" which gives us access to the command line arguments
use std::env;

// ====================
// MAIN FUNCTION
// ====================

fn main() {

    // New growable array called "numbers"
    let mut numbers = Vec::new();

    // Set the variable "arg" to each argument
    // "std::env" returns an iterator, whose first argument is the name of the program
    // being run. This is not useful for us, so we skip it using ".skip(1)"
    for arg in env::args().skip(1) {

        // Parse command-line args as an u64 int
        // "from_str()" function returns a "Result" value. With "expect" we return the
        // parsed value and append if the operation was successful, or we exit the program 
        // and print a message if the operation failed.
        numbers.push(u64::from_str(&arg).expect("Error parsing argument."));
    }

    // If no arguments were provided, print to the standard error and
    // inmediately exit the process.
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER1 NUMBER2 ...");
        std::process::exit(1);
    }

    // We take the initial greatest common denominator as the first
    // element of the vector.
    let mut d = numbers[0];

    // Rust is cautious when handling types with variable sizes (eg. vectors): It wants to 
    // leave the programmer in control over memory consumption, making it clear how long 
    // each value lives, while still ensuring memory is freed promptly when no longer needed.

    // Borrow a reference to the vector's elements. The reference is stored in "m"
    // When we iterate, we want to tell Rust that ownership of the vector should remain with
    // the variable "numbers", we are just borrowing its elements for the loop.
    for m in &numbers[1..] {

        // The "*" operator dereferences "m" yielding the value it refers to
        // (retrieves the actual value from the reference)
        d = gcd(d, *m);

        // NOTE: When we reach the end of "main", Rust automatically frees the value of "numbers"
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);

    // NOTE: If Rust reaches the end of the main function, it assumes that the execution
    // was successful. An execution can be clasified as "failed" only if we explicitly call
    // functions like "expect" or "std::process::exit"
}

// ====================

// Finds the Greatest Common Divisor of two integers
// (Created using Euclid's algorithm)
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5  * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}