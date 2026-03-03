pub mod bip39;
pub mod crypto;
pub mod recovery;

use anyhow::{anyhow, Result};
use std::time::Duration;

pub use recovery::{RecoveryEngine, RecoveryOptions, RecoveryResult};

#[derive(Debug, Clone, Copy)]
pub enum WordlistLang {
    English,
}

/// Validate a seed phrase
pub fn validate_phrase(phrase: &str) -> Result<bool> {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    
    // Check word count
    if ![12, 15, 18, 21, 24].contains(&words.len()) {
        return Ok(false);
    }

    // Check if all words are in BIP39 wordlist
    let wordlist = bip39::get_wordlist();
    for word in &words {
        if !wordlist.contains(&word.to_string()) {
            return Ok(false);
        }
    }

    // Validate checksum
    bip39::validate_checksum(&words)
}

/// Derive addresses from a seed phrase
pub fn derive_addresses(phrase: &str, count: usize) -> Result<Vec<String>> {
    if !validate_phrase(phrase)? {
        return Err(anyhow!("Invalid seed phrase"));
    }

    crypto::derive_addresses(phrase, count)
}

/// Suggest similar words based on edit distance
pub fn suggest_words(word: &str, max_distance: usize) -> Result<Vec<String>> {
    let wordlist = bip39::get_wordlist();
    let mut suggestions = Vec::new();

    for dict_word in &wordlist {
        let distance = levenshtein_distance(word, dict_word);
        if distance <= max_distance {
            suggestions.push(dict_word.clone());
        }
    }

    suggestions.sort_by_key(|w| levenshtein_distance(word, w));
    Ok(suggestions)
}

/// Generate a random valid seed phrase
pub fn generate_phrase(word_count: usize) -> Result<String> {
    if ![12, 15, 18, 21, 24].contains(&word_count) {
        return Err(anyhow!("Invalid word count. Must be 12, 15, 18, 21, or 24"));
    }

    bip39::generate_mnemonic(word_count)
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,
                    matrix[i + 1][j] + 1
                ),
                matrix[i][j] + cost
            );
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_phrase() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        assert!(validate_phrase(phrase).unwrap());
    }

    #[test]
    fn test_validate_invalid_length() {
        let phrase = "abandon abandon abandon";
        assert!(!validate_phrase(phrase).unwrap());
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("abandon", "abandon"), 0);
        assert_eq!(levenshtein_distance("test", "test"), 0);
    }

    #[test]
    fn test_suggest_words() {
        let suggestions = suggest_words("abandn", 2).unwrap();
        assert!(suggestions.contains(&"abandon".to_string()));
    }
}
