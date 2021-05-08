const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_vowel(letter: &char) -> bool {
    VOWELS.iter().any(|vowel| vowel == letter)
}

fn pig_latinize(word: &str) -> String {
    match word.chars().nth(0) {
        Some(letter) => {
            if is_vowel(&letter) {
                format!("{}-hay", word)
            } else {
                if word.len() == 1 {
                    format!("{}ay", word)
                } else {
                    format!("{}-{}ay", &String::from(word)[1..], letter)
                }
            }
        }
        None => String::from(word),
    }
}

fn pig_latin(text: &str) -> String {
    let new_text = text
        .split(" ")
        .map(pig_latinize)
        .collect::<Vec<String>>()
        .join(" ");
    new_text
}

fn main() {
    let text = "bonjour les enfants";
    let latinized = pig_latin(text);

    println!("{}", latinized);
}
