mod crabby_caesar;
use std::io;

//for debugging purposes
/*fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}*/

fn main() {
    // Purpose:    IO, and calls your functions.
    // Parameters: None
    // User Input: Input text to translate
    // Prints:     Print result
    // Returns:    Nothing
    // Modifies:   Nothing outside its scope
    // Calls:      std::
    // Tests:      stdio_tests/
    // Status:     Do this one.
    // The string to be encrypted/decrypted:
    println!("Enter a string to translate:");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");
    let message: String = message.trim().to_string(); // Trim newline character, convert to string

    // Whether the program encrypts or decrypts:
    println!("encrypt or decrypt?");
    let mut _mode = String::new();
    io::stdin()
        .read_line(&mut _mode)
        .expect("Failed to read line");
    let _mode = _mode.trim().to_string(); // Trim newline character, convert to string

    // The encryption/decryption key:
    println!("What is your key?");
    let mut _key = String::new();
    io::stdin()
        .read_line(&mut _key)
        .expect("Failed to read line");
    let _key: isize = _key.trim().parse().expect("Please type a number!");

    // Stores the encrypted/decrypted form of the message:
    let _translated: String = crabby_caesar::caesar_translate(message, _mode, _key);
    println!("{}\n", _translated) //the \n bruh
}
