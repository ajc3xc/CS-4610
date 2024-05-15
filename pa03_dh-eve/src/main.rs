//welcome to the danger zone

use std::env;
use std::fs;
use std::io::{self};
mod eve;

/*
fn read_and_print_lines(n: usize) {
    let mut input_lines = Vec::new(); // Vector to hold the input lines

    for _ in 0..n {
        let mut input = String::new(); // Create a new String to hold the input
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
        io::stdin().read_line(&mut input) // Read a line of input
            .expect("Failed to read line"); // Handle potential errors

        // Trim the newline character from the input and add it to the vector
        input_lines.push(input.trim_end().to_string());
    }

    println!("\nYou entered the following lines:");
    for line in input_lines {
        println!("{}", line); // Print each line
    }
}
*/

fn main() {
    // Purpose:    Driver for DH.
    // Parameters: None
    // User Input: If no args, input dec numbers
    // Prints:     If no args, then print result
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      ?
    // Tests:      arg_tests/ and stdio_tests/
    // Status:     Student does this
    let args: Vec<String> = env::args().collect();

    //println!("{:?}", &args);

    let alice_broadcast: u64;
    let bob_broadcast: u64;
    let public_base: u64;
    let public_modulus: u64;

    if args.len() == 3 {
        let input_file_path = &args[1];

        let input_file =
            fs::read_to_string(input_file_path).expect("Failed to read input file path");
        let input_arr: Vec<u64> = input_file
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        alice_broadcast = input_arr[0];
        bob_broadcast = input_arr[1];
        public_base = input_arr[2];
        public_modulus = input_arr[3];
    } else {
        //alice broadcasts
        let mut alice_broadcast_input = String::new();
        io::stdin()
            .read_line(&mut alice_broadcast_input)
            .expect("Failed to read line");
        alice_broadcast = alice_broadcast_input
            .trim()
            .parse()
            .expect("Alice's broadcast is invalid");

        //alice broadcasts
        let mut bob_broadcast_input = String::new();
        io::stdin()
            .read_line(&mut bob_broadcast_input)
            .expect("Failed to read line");
        bob_broadcast = bob_broadcast_input
            .trim()
            .parse()
            .expect("Bob's broadcast is invalid");

        //bob secret
        let mut public_base_input = String::new();
        io::stdin()
            .read_line(&mut public_base_input)
            .expect("Failed to read line");
        public_base = public_base_input
            .trim()
            .parse()
            .expect("Bob's secret is invalid");

        //bob secret
        let mut public_modulus_input = String::new();
        io::stdin()
            .read_line(&mut public_modulus_input)
            .expect("Failed to read line");
        public_modulus = public_modulus_input
            .trim()
            .parse()
            .expect("Bob's secret is invalid");
    }
    //for checking input is right
    //println!("{} {} {} {}", alice_broadcast, bob_broadcast, bob_secret, public_modulus);
    //let baby_results: [u64; 3] = eve::baby_eve(alice_broadcast, bob_broadcast, public_base);
    let results: [u64; 3] =
        eve::big_eve(alice_broadcast, bob_broadcast, public_base, public_modulus);

    //print results
    //output results to a file
    if args.len() == 3 {
        let output_file_path = &args[2];
        //convert array to string
        let numbers_string = format!("{} {} {}", results[0], results[1], results[2]);
        //output to output file
        fs::write(output_file_path, numbers_string)
            .expect("Failed to write results to output file");
    } else {
        println!("{} {} {}", results[0], results[1], results[2]);
    }
}
