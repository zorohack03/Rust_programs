fn extract_word(sentence: &str, start: usize, end: usize) -> &str {
    &sentence[start..end]
}

fn main() {
    let sentence = "Rust is fast and safe.";
    let word = extract_word(sentence, 0, 4); // Extract "Rust"
    println!("Extracted word: {}", word);
}

