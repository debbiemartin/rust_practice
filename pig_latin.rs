fn main() {
    let word = String::from("first");
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    let word_minus_first_char = chars.as_str();
    
    println!("{}{}-ay", word_minus_first_char, first_char)
}
