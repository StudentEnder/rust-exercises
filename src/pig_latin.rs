pub fn format_pig_latin(text: &String) -> String {
    let mut converted_text = String::new();
    
    for word in text.split_whitespace() {
        let mut word = word.to_string();

        let first_char: char = word.chars().next().expect("Expected char type");

        use is_vowel::IsRomanceVowel;

        if first_char.is_romance_vowel() {
            word.push_str("hay ");
        } else {
            word.remove(0);
            word = format!("{word}{first_char}ay ");
        }

        converted_text.push_str(&word)
    }

    converted_text
}