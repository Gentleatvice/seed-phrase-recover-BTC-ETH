use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::time::{Duration, Instant};

use crate::bip39;
use crate::crypto;
use crate::WordlistLang;

#[derive(Debug, Clone)]
pub struct RecoveryOptions {
    pub phrase: String,
    pub target_address: Option<String>,
    pub threads: usize,
    pub derivation_path: Option<String>,
    pub crypto_type: String,
    pub lang: WordlistLang,
}

#[derive(Debug, Clone)]
pub struct RecoveryResult {
    pub phrase: String,
    pub address: String,
    pub attempts: usize,
    pub duration: Duration,
}

pub struct RecoveryEngine {
    options: RecoveryOptions,
    wordlist: Vec<String>,
    missing_positions: Vec<usize>,
    known_words: Vec<Option<String>>,
}

impl RecoveryEngine {
    pub fn new(options: RecoveryOptions) -> Result<Self> {
        let wordlist = bip39::get_wordlist();
        
        // Parse phrase and find missing positions
        let words: Vec<&str> = options.phrase.split_whitespace().collect();
        let mut missing_positions = Vec::new();
        let mut known_words = Vec::new();

        for (i, word) in words.iter().enumerate() {
            if *word == "???" || *word == "?" || *word == "_" {
                missing_positions.push(i);
                known_words.push(None);
            } else {
                if !wordlist.contains(&word.to_string()) {
                    return Err(anyhow!("Word '{}' is not in BIP39 wordlist", word));
                }
                known_words.push(Some(word.to_string()));
            }
        }

        if missing_positions.is_empty() {
            return Err(anyhow!("No missing words found. Use '???' to mark unknown positions"));
        }

        if missing_positions.len() > 4 {
            return Err(anyhow!("Too many missing words ({}). Maximum is 4 for reasonable recovery time", missing_positions.len()));
        }

        Ok(Self {
            options,
            wordlist,
            missing_positions,
            known_words,
        })
    }

    pub fn recover(&mut self) -> Result<RecoveryResult> {
        let start_time = Instant::now();
        let attempts = Arc::new(AtomicUsize::new(0));
        let found = Arc::new(AtomicBool::new(false));

        println!("Missing {} word(s) at positions: {:?}", 
                 self.missing_positions.len(), 
                 self.missing_positions);
        
        let total_combinations = self.wordlist.len().pow(self.missing_positions.len() as u32);
        println!("Total combinations to try: {}", total_combinations);
        println!();

        // Generate all possible combinations
        let combinations = self.generate_combinations();
        
        // Try to find matching phrase
        let result = combinations
            .par_iter()
            .find_map_any(|words| {
                if found.load(Ordering::Relaxed) {
                    return None;
                }

                let current_attempts = attempts.fetch_add(1, Ordering::Relaxed);
                
                if current_attempts % 10000 == 0 {
                    println!("Tried {} combinations...", current_attempts);
                }

                // Build complete phrase
                let mut phrase_words = self.known_words.clone();
                for (idx, pos) in self.missing_positions.iter().enumerate() {
                    phrase_words[*pos] = Some(words[idx].clone());
                }

                let phrase: Vec<String> = phrase_words.iter()
                    .filter_map(|w| w.clone())
                    .collect();

                // Validate checksum
                let phrase_refs: Vec<&str> = phrase.iter().map(|s| s.as_str()).collect();
                if let Ok(valid) = bip39::validate_checksum(&phrase_refs) {
                    if !valid {
                        return None;
                    }

                    let phrase_string = phrase.join(" ");

                    // If no target address, return first valid phrase
                    if self.options.target_address.is_none() {
                        found.store(true, Ordering::Relaxed);
                        
                        let address = crypto::derive_addresses(&phrase_string, 1)
                            .ok()?
                            .first()?
                            .clone();

                        return Some(RecoveryResult {
                            phrase: phrase_string,
                            address,
                            attempts: current_attempts,
                            duration: start_time.elapsed(),
                        });
                    }

                    // Check if derived address matches target
                    if let Ok(addresses) = crypto::derive_addresses(&phrase_string, 1) {
                        if let Some(address) = addresses.first() {
                            if let Some(ref target) = self.options.target_address {
                                if address == target {
                                    found.store(true, Ordering::Relaxed);
                                    
                                    return Some(RecoveryResult {
                                        phrase: phrase_string,
                                        address: address.clone(),
                                        attempts: current_attempts,
                                        duration: start_time.elapsed(),
                                    });
                                }
                            }
                        }
                    }
                }

                None
            });

        match result {
            Some(res) => Ok(res),
            None => Err(anyhow!("Could not recover seed phrase after {} attempts", 
                               attempts.load(Ordering::Relaxed))),
        }
    }

    fn generate_combinations(&self) -> Vec<Vec<String>> {
        let missing_count = self.missing_positions.len();
        
        match missing_count {
            1 => self.generate_single_missing(),
            2 => self.generate_double_missing(),
            _ => self.generate_multiple_missing(),
        }
    }

    fn generate_single_missing(&self) -> Vec<Vec<String>> {
        self.wordlist.iter()
            .map(|word| vec![word.clone()])
            .collect()
    }

    fn generate_double_missing(&self) -> Vec<Vec<String>> {
        let mut combinations = Vec::new();
        
        for word1 in &self.wordlist {
            for word2 in &self.wordlist {
                combinations.push(vec![word1.clone(), word2.clone()]);
            }
        }
        
        combinations
    }

    fn generate_multiple_missing(&self) -> Vec<Vec<String>> {
        let mut combinations = vec![vec![]];
        
        for _ in 0..self.missing_positions.len() {
            let mut new_combinations = Vec::new();
            
            for combo in combinations {
                for word in &self.wordlist {
                    let mut new_combo = combo.clone();
                    new_combo.push(word.clone());
                    new_combinations.push(new_combo);
                }
            }
            
            combinations = new_combinations;
        }
        
        combinations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_engine_creation() {
        let options = RecoveryOptions {
            phrase: "abandon abandon ??? abandon abandon abandon abandon abandon abandon abandon abandon about".to_string(),
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
    fn test_too_many_missing() {
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
}
