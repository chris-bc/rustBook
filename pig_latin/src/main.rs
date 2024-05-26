use std::io;

fn is_vowel(c: char) -> bool {
    // What a mess! It'd almost be better to just compare upper and lower case values
    // TODO: Use match to filter out None values instead of using a default '_'
    let c = c.to_lowercase().to_string().chars().nth(0).unwrap_or('_');
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn main() {
    // Get some text from the user
    println!("Enter text for translation:");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to get user input");
    for word in sentence.split_whitespace() {
        if is_vowel(word.chars().nth(0).unwrap_or('_')) {
            // Result is word[1..] + "-hay"
            println!("{}-hay", &word[1..]);
        } else {
            // Result is word[1..] + "-" + word[0] + "ay"
            println!("{}-{}ay", &word[1..], word.chars().nth(0).unwrap_or('_'));
        }
    }
}
