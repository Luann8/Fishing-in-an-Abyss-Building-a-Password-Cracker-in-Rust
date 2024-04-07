use std::fs::File;
use std::io::{BufRead, BufReader};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

fn derive(iterations: u32, salt: &str, password: &str) -> String {
    let mut salt_vec = Vec::new();
    for i in 0..(salt.len() / 2) {
        let byte = u8::from_str_radix(&salt[2 * i..2 * i + 2], 16).unwrap();
        salt_vec.push(byte);
    }

    let mut derived_hash: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        DIGEST_ALG,
        iterations,
        &salt_vec,
        password.as_bytes(),
        &mut derived_hash,
    );

    let mut lower = String::new();
    for &byte in derived_hash.iter() {
        write!(&mut lower, "{:02x}", byte).expect("Unable to write byte");
    }
    lower
}

fn guess(password_guess: &str, iterations: u32, salt: &str, derived: &str) -> bool {
    derive(iterations, salt, password_guess) == derived
}

fn run_crack(given_iterations: u32, given_salt: &str, given_derived: &str) -> Option<String> {
    let words = make_word_list("agile_words.txt");

    for word1 in &words {
        for word2 in &words {
            for word3 in &words {
                let password_guess = format!("{} {} {}", word1, word2, word3);
                if guess(&password_guess, given_iterations, given_salt, given_derived) {
                    println!("Found it! {}", password_guess);
                    return Some(password_guess);
                }
            }
        }
    }
    None
}

fn main() {
    // Informações do desafio
    let given_iterations = 100000;
    let given_salt = "8ad1712ab5d632d8c4dac07b792ebb17";
    let given_derived = "a3a8b8eb8e739c86f67332d17364b149cd88f33bb11eedae066ac366711ec266";

    // Executa a quebra da senha
    run_crack(given_iterations, given_salt, given_derived);
}
