use rand::{thread_rng, Rng};
use std::env;
use std::fs::{write};

pub fn generate_keys(num_keys: usize) -> Vec<u8> {
    // Purpose:    Gets a random vector of bytes, with each key being 2048 bits
    // Parameters: The number of keys
    // User Input: None
    // Prints:     Nothing
    // Returns:    A Vec<u8> containing the bytes of num_keys keys
    // Modifies:   Nothing
    // Calls:      std:: , rand::
    // Tests:      unit_tests/
    // Status:     Do this one.
    //create empty vector of capacity 50 * number of bytes in 2048 bit key (256)
    let total_capacity: usize = num_keys * 256;
    let mut keys: Vec<u8> = Vec::with_capacity(num_keys);

    //generate rng seed once
    //generate random 2048 bit number and return it
    let mut rng = thread_rng();
    for _ in 0..total_capacity {
        keys.push(rng.gen());
    }
    return keys;
}

fn main() {
    // Purpose:    Parses the args for this file, and writes the keyfile
    // Parameters: None
    // User Input: None
    // Prints:     Nothing
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      custom_test (see grade.sh)
    // Status:     Do this one.
    println!("Delete this and write your code here.");
    //get all arguments
    let inputs: Vec<String> = env::args().collect();
    let output_path: &String = &inputs[1]; // get first argument as a string
                                           // Skip the first argument, which is the path to the program
    let keys = generate_keys(50);

    //write to file
    let _ = write(output_path, keys);
}
