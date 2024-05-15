use std::env;
use std::{
    fs::{read_to_string, write, File},
    io::{self, Read, Seek},
};

//for debugging purposes
/*
fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}*/

fn main() {
    // Purpose:    Parses args, reads/writes files, calls apply_key
    // Parameters: None
    // User Input: None
    // Prints:     Nothing
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      arg_tests/
    // Status:     Do this one.
    let inputs: Vec<String> = env::args().collect();
    let code_book_path: &String = &inputs[1];
    let key_num: usize = inputs[2].parse().expect("Failed");
    let input_file_path: &String = &inputs[3];
    let output_path: &String = &inputs[4];

    let mut code_book = File::open(code_book_path).unwrap();

    // Skip to the fourth key (2048 bits * 3 = 6144 bytes)
    //nth key starts at n * 256th bit.
    code_book
        .seek(io::SeekFrom::Start((key_num * 256) as u64))
        .unwrap();

    // Read the fourth key (2048 bits = 256 bytes)
    let mut key = [0u8; 256];
    code_book.read_exact(&mut key).unwrap();

    let key_vec: Vec<u8> = key.to_vec();

    let in_str = read_to_string(input_file_path).unwrap();

    let result = apply_key(&key_vec, &in_str);

    let _ = write(output_path, result.as_bytes());
}

pub fn apply_key(key: &Vec<u8>, in_str: &String) -> String {
    // Purpose:    Applies OTP to the in_str based on the key
    // Parameters: A vector of bytes and a unicode string of equal length by chars
    // User Input: None
    // Prints:     Nothing
    // Returns:    A std::String of the same character length as the input string
    // Modifies:   Nothing
    // Calls:      std::
    // Tests:      unit_tests/
    // Status:     Do this one.
    let mut new_message = String::new();

    for (i, char) in in_str.chars().enumerate().take(256) {
        // Ensure we don't exceed the key length, cycling through the key if necessary
        let key_byte = key[i % key.len()];
        // Perform XOR operation between the character code and the key byte
        let new_char = (char as u8) ^ key_byte;
        let safe_new_char = new_char;
        // Convert the result back to a char and append to the new message
        // This assumes the result is within the ASCII range
        if let Some(decoded_char) = std::char::from_u32(safe_new_char as u32) {
            new_message.push(decoded_char);
        } else {
            // Handle the case where the XOR result isn't a valid char
            new_message.push('?'); // Placeholder for invalid chars
        }
    }
    return new_message;
}
