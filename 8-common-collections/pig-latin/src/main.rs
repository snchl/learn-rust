use std::io;

const VOWEL: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
const CONSONANT: [char; 20] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'z',
];

fn main() {
    println!("Welcome in the pig-latin word converter!");
    println!("Enter the word(s) you wish to convert in pig-latin");

    let mut original_words = String::new();

    io::stdin()
        .read_line(&mut original_words)
        .expect("Failed to read line");

    // Remove newline char
    original_words.pop();

    println!("Your original word(s) is '{}'", original_words);

    let mut converted_words = String::new();

    for word in original_words.split_whitespace() {
        print!("{}", word); // TODO
        let first_char = word.to_lowercase().chars().next().expect("Failed to get first chard");
        print!(" - {}", first_char); // TODO

        let mut converted_word = String::new();

        if VOWEL.contains(&first_char) {
            print!(" - VOWEL");
            converted_word = format!("{word}-hay");
        }

        if CONSONANT.contains(&first_char) {
            print!(" - CONSONANT");
            let mut trunc_word = String::from(word);
            let first_consonant = trunc_word.remove(0);
            converted_word = format!("{trunc_word}-{first_consonant}ay");
        }

        println!(" - {}", converted_word);

        converted_words.push_str(&converted_word);
        converted_words.push_str(" ");
    }

    println!("Your converted word(s) is '{}'", converted_words);
}
