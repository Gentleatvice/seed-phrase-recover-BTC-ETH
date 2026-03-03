use seed_recovery::{validate_phrase, suggest_words, generate_phrase};

#[test]
fn test_phrase_validation() {
    // Valid 12-word phrase
    let valid_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    assert!(validate_phrase(valid_phrase).unwrap());

    // Invalid word count
    let invalid_phrase = "abandon abandon abandon";
    assert!(!validate_phrase(invalid_phrase).unwrap());

    // Invalid word
    let invalid_word_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon invalid";
    assert!(!validate_phrase(invalid_word_phrase).unwrap());
}

#[test]
fn test_word_suggestions() {
    let result = suggest_words("abandn", 2);
    assert!(result.is_ok());
    
    let suggestions = result.unwrap();
    assert!(suggestions.contains(&"abandon".to_string()));
}

#[test]
fn test_phrase_generation() {
    for word_count in [12, 15, 18, 21, 24] {
        let result = generate_phrase(word_count);
        assert!(result.is_ok());
        
        let phrase = result.unwrap();
        let words: Vec<&str> = phrase.split_whitespace().collect();
        assert_eq!(words.len(), word_count);
    }
}

#[test]
fn test_invalid_word_count_generation() {
    let result = generate_phrase(10);
    assert!(result.is_err());
}

#[test]
fn test_recovery_engine_initialization() {
    use seed_recovery::{RecoveryEngine, RecoveryOptions, WordlistLang};
    
    let options = RecoveryOptions {
        phrase: "abandon ??? abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string(),
        target_address: None,
        threads: 1,
        derivation_path: None,
        crypto_type: "bitcoin".to_string(),
        lang: WordlistLang::English,
    };

    let engine = RecoveryEngine::new(options);
    assert!(engine.is_ok());
}

#[test]
fn test_recovery_no_missing_words() {
    use seed_recovery::{RecoveryEngine, RecoveryOptions, WordlistLang};
    
    let options = RecoveryOptions {
        phrase: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string(),
        target_address: None,
        threads: 1,
        derivation_path: None,
        crypto_type: "bitcoin".to_string(),
        lang: WordlistLang::English,
    };

    let engine = RecoveryEngine::new(options);
    assert!(engine.is_err());
}

#[test]
fn test_recovery_too_many_missing() {
    use seed_recovery::{RecoveryEngine, RecoveryOptions, WordlistLang};
    
    let options = RecoveryOptions {
        phrase: "??? ??? ??? ??? ??? abandon abandon abandon abandon abandon abandon about".to_string(),
        target_address: None,
        threads: 1,
        derivation_path: None,
        crypto_type: "bitcoin".to_string(),
        lang: WordlistLang::English,
    };

    let engine = RecoveryEngine::new(options);
    assert!(engine.is_err());
}
