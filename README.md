## Password Cracking Tool Instructions

### Overview:
This Rust program attempts to crack a password hashed using PBKDF2 with a given salt and derived hash. It reads a list of words from a file, generates combinations of these words, and derives hashes to compare with the given derived hash. If a match is found, it prints the password.

### Instructions:
1. **Wordlist Preparation:**
   - Prepare a text file containing a list of words to be used for password guessing. Each word should be on a separate line. Save this file with the name `agile_words.txt`.

2. **Setup Rust Environment:**
   - Ensure you have Rust installed on your system. If not, you can download and install it from [Rust's official website](https://www.rust-lang.org/tools/install).

3. **Compile and Run:**
   - Copy the provided code into a Rust source file (e.g., `main.rs`).
   - Open a terminal or command prompt and navigate to the directory containing your Rust source file.
   - Run the following command to compile and execute the program:
     ```bash
     cargo run
     ```
   - The program will attempt to crack the password using the given salt and derived hash. If successful, it will print the password.

4. **Adjust Parameters (Optional):**
   - You can adjust the `given_iterations`, `given_salt`, and `given_derived` variables in the `main` function to match the parameters of the password you want to crack.

5. **Optimization (Optional):**
   - You may consider optimizing the program by parallelizing the password guessing process or using a more efficient algorithm for wordlist manipulation.

6. **Wordlist Considerations:**
   - Ensure that the wordlist file (`agile_words.txt`) contains a diverse set of words that might be used in passwords.

7. **Caution:**
   - Use this tool responsibly and only for legitimate purposes. Unauthorized use may violate privacy and security laws.

### Notes:
- This program is for educational purposes and should not be used for illegal activities.
- Ensure you have permission to crack the password you are attempting to guess.
