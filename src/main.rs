use rand::{thread_rng, Rng};

fn main() {
    let signal = 16;
    let password = generate_secure_password(signal);
    println!("Generated secure password: {}", password);
}

fn generate_secure_password(length: usize) -> String {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // O removed
    let numbers   = "1234567890";                 // 0 removed
    let symbols   = "!@#$%^&*()-_=+[]{};<>?";

    let letters = format!("{}{}", lowercase, uppercase);
    let all_chars = format!("{}{}{}", letters, numbers, symbols);
    let charset: Vec<char> = all_chars.chars().collect();

    let mut rng = thread_rng();

    loop {
        let mut password: Vec<char> = Vec::new();

        // Guarantee at least one of each required type
        password.push(letters.chars().nth(rng.gen_range(0..letters.len())).unwrap());
        password.push(numbers.chars().nth(rng.gen_range(0..numbers.len())).unwrap());
        password.push(symbols.chars().nth(rng.gen_range(0..symbols.len())).unwrap());

        // Fill the rest randomly
        for _ in 3..length {
            let idx = rng.gen_range(0..charset.len());
            password.push(charset[idx]);
        }

        // Shuffle to avoid predictable character positions
        use rand::seq::SliceRandom;
        password.shuffle(&mut rng);

        let password_str: String = password.iter().collect();

        // Extra safety: avoid both 'O' and '0' (but '0' is already excluded)
        if !password_str.contains('O') && !password_str.contains('0') {
            return password_str;
        }
    }
}
