pub fn pig_latin(word: &str) -> String {
    match word.chars().nth(0).expect("length can not be zero") {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", &word[word.chars().nth(0).unwrap().len_utf8()..], word.chars().nth(0).unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_word_starting_with_vowel_to_hay()
    {
        let word = "apple";

        assert_eq!(pig_latin(word), String::from("apple-hay"));
    }

    #[test]
    fn transform_word_starting_with_consonant_to_consonant_plus_ay()
    {
        let word = "fable";

        assert_eq!(pig_latin(word), String::from("able-fay"));
    }

    #[test]
    #[should_panic]
    fn transform_word_starting_should_panic_when_empty()
    {
        let word = "";
        pig_latin(word);
    }
}