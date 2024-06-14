pub fn palindrome(word: String) {
    let palindrome = (word.to_string()).chars().rev().collect::<String>();

    if palindrome == word {
        println!("{} is a palindrome of {}", word, palindrome);
    } else {
        println!("{} is not palindrome of {}", word, palindrome);
    }
}