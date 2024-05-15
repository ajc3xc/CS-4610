pub fn baby_eve(alice_broadcasts: u64, bob_broadcasts: u64, public_base: u64) -> [u64; 3] {
    // Purpose:     Crack baby DH
    // Parameters:  alice's broadcast, bob's broadcast, and the public base
    // User-input:  None
    // Prints:      Nothing
    // Returns:     Should return an array of 3 unsigned ints:
    //              Alice's secret, Bob's secret, shared secret
    // Modifies:    Nothing
    // Calls:       ?
    // Tests:       unit_test_babydh.rs
    // Status:      Done, correct, but is it ideal?
    let alice_broadcast_f64 = alice_broadcasts as f64;
    let bob_broadcast_f64 = bob_broadcasts as f64;
    let public_base_f64 = public_base as f64;

    //say alice_broadcast = (shared_base)^(alic_secret) = 2^5 = 32
    //2 = root5(32), log2(32) = ln(32) / ln(2) = 5
    let alice_secret: f64 = alice_broadcast_f64.ln() / public_base_f64.ln();
    //same with bob_secret
    let bob_secret: f64 = bob_broadcast_f64.ln() / public_base_f64.ln();

    //shared secret = bob_broadcasts^(alice_secret) or alice_broadcast^(bob_secret)
    let shared_secret: f64 = alice_broadcast_f64.powf(bob_secret);

    [alice_secret as u64, bob_secret as u64, shared_secret as u64]
}

pub fn big_eve(
    alice_broadcasts: u64,
    bob_broadcasts: u64,
    public_base: u64,
    public_modulus: u64,
) -> [u64; 3] {
    // Purpose:      Crack real DH (albeit not with huge numbers)
    // Parameters:   Alice's broadcast, Bob's broadcast, the public base and modulus of DH.
    // User-input:   None
    // Prints:       Nothing
    // Returns:      Should return an array of 3 ints:
    //               Alice's secret, Bob's secret, shared secret
    // Modifies:     Nothing
    // Calls:        ?
    // Test:         ./unit_tests/unit_test_babydh.rs
    // Status:       TODO delete the 0 placeholders, and replace with real computations
    let (mut alice_secret, mut bob_secret): (f64, f64) = (0.0, 0.0);
    let shared_secret;

    //brute force both at once
    //
    //(public_base ^ alice_secret) % public_modulus = alice_broadcast
    //alice broadcast is public key.
    //know public_base, public_modulus and alice_broadcast, so need to keep incrementing
    //until you guess secret alice_secret.
    for i in 1..public_modulus {
        let guess = (public_base as f64)
            .powf(i as f64)
            .rem_euclid(public_modulus as f64);

        //check if guess == alice_secret
        if guess == alice_broadcasts as f64 {
            alice_secret = i as f64;
            //check if both keys have been found
            if bob_secret != 0.0 {
                break;
            }
        }
        //check if guess == bob_secret
        if guess == bob_broadcasts as f64 {
            bob_secret = i as f64;
            //check if both keys have been found
            if alice_secret != 0.0 {
                break;
            }
        }
    }

    let cracked_almost = (public_base as f64).powf((alice_secret * bob_secret) as f64);
    shared_secret = ((cracked_almost as u64) % public_modulus) as f64;

    [alice_secret as u64, bob_secret as u64, shared_secret as u64]

    //[0, 0, 0]
}
