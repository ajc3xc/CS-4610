pub fn caesar_translate(input_text: String, mode: String, key: isize) -> String {
    // Purpose:    Does the Caesar cipher logic.
    // Parameters: input_text to translate,
    //             mode (encrypt or decrypt), and
    //             key (in size).
    // User Input: None.
    // Prints:     None.
    // Returns:    Translated text as a std::String.
    // Modifies:   Nothing.
    // Calls:      Pure rust, no imports. Hint: rem_euclid
    // Tests:      ./unit_tests/*
    // Status:     Do this one!
    // asserteq!(caesar_translate("abc".to_string(), "encrypt".to_string(), 1), "bcd".to_string())
    // asserteq!(caesar_translate("bcd".to_string(), "decrypt".to_string(), 1), "abc".to_string())
    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

    let mut translated = String::new();

    for symbol in input_text.chars() {
        // Note: Only symbols in the `symbols` string can be encrypted/decrypted.
        if let Some(symbol_index) = symbols.find(symbol) {
            let symbols_len = symbols.len() as isize;

            // Perform encryption/decryption:
            let translated_index = match mode.as_str() {
                "encrypt" => (symbol_index as isize + key).rem_euclid(symbols_len),
                "decrypt" => (symbol_index as isize - key).rem_euclid(symbols_len),
                _ => panic!("Invalid"),
            };

            translated.push(
                symbols
                    .chars()
                    .nth(translated_index as usize)
                    .expect("Invalid index"),
            );
        } else {
            // Append the symbol without encrypting/decrypting:
            translated.push(symbol);
        }
    }

    return translated;
}
