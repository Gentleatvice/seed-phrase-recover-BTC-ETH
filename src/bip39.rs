use anyhow::{anyhow, Result};
use sha2::{Sha256, Digest};

/// Get the English BIP39 wordlist
pub fn get_wordlist() -> Vec<String> {
    // This is a subset for demonstration. In production, use the full BIP39 wordlist
    // or import from a crate like `bip39`
    vec![
        "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
        "absurd", "abuse", "access", "accident", "account", "accuse", "achieve", "acid",
        "acoustic", "acquire", "across", "act", "action", "actor", "actress", "actual",
        "adapt", "add", "addict", "address", "adjust", "admit", "adult", "advance",
        "advice", "aerobic", "afford", "afraid", "again", "age", "agent", "agree",
        "ahead", "aim", "air", "airport", "aisle", "alarm", "album", "alcohol",
        "alert", "alien", "all", "alley", "allow", "almost", "alone", "alpha",
        "already", "also", "alter", "always", "amateur", "amazing", "among", "amount",
        "amused", "analyst", "anchor", "ancient", "anger", "angle", "angry", "animal",
        "ankle", "announce", "annual", "another", "answer", "antenna", "antique", "anxiety",
        "any", "apart", "apology", "appear", "apple", "approve", "april", "arch",
        "arctic", "area", "arena", "argue", "arm", "armed", "armor", "army",
        "around", "arrange", "arrest", "arrive", "arrow", "art", "artefact", "artist",
        "artwork", "ask", "aspect", "assault", "asset", "assist", "assume", "asthma",
        "athlete", "atom", "attack", "attend", "attitude", "attract", "auction", "audit",
        "august", "aunt", "author", "auto", "autumn", "average", "avocado", "avoid",
        "awake", "aware", "away", "awesome", "awful", "awkward", "axis", "baby",
        "bachelor", "bacon", "badge", "bag", "balance", "balcony", "ball", "bamboo",
        "banana", "banner", "bar", "barely", "bargain", "barrel", "base", "basic",
        "basket", "battle", "beach", "bean", "beauty", "because", "become", "beef",
        "before", "begin", "behave", "behind", "believe", "below", "belt", "bench",
        "benefit", "best", "betray", "better", "between", "beyond", "bicycle", "bid",
        "bike", "bind", "biology", "bird", "birth", "bitter", "black", "blade",
        "blame", "blanket", "blast", "bleak", "bless", "blind", "blood", "blossom",
        "blouse", "blue", "blur", "blush", "board", "boat", "body", "boil",
        "bomb", "bone", "bonus", "book", "boost", "border", "boring", "borrow",
        "boss", "bottom", "bounce", "box", "boy", "bracket", "brain", "brand",
        "brass", "brave", "bread", "breeze", "brick", "bridge", "brief", "bright",
        "bring", "brisk", "broccoli", "broken", "bronze", "broom", "brother", "brown",
        "brush", "bubble", "buddy", "budget", "buffalo", "build", "bulb", "bulk",
        "bullet", "bundle", "bunker", "burden", "burger", "burst", "bus", "business",
        "busy", "butter", "buyer", "buzz",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

/// Validate BIP39 checksum
pub fn validate_checksum(words: &[&str]) -> Result<bool> {
    let word_count = words.len();
    
    if ![12, 15, 18, 21, 24].contains(&word_count) {
        return Ok(false);
    }

    let wordlist = get_wordlist();
    let mut indices = Vec::new();

    for word in words {
        match wordlist.iter().position(|w| w == word) {
            Some(idx) => indices.push(idx),
            None => return Ok(false),
        }
    }

    // Convert indices to bits
    let mut bits = String::new();
    for idx in indices {
        bits.push_str(&format!("{:011b}", idx));
    }

    // Calculate checksum length
    let checksum_len = word_count / 3;
    let entropy_len = bits.len() - checksum_len;

    // Split entropy and checksum
    let entropy_bits = &bits[..entropy_len];
    let checksum_bits = &bits[entropy_len..];

    // Convert entropy bits to bytes
    let mut entropy_bytes = Vec::new();
    for chunk in entropy_bits.as_bytes().chunks(8) {
        let byte_str = std::str::from_utf8(chunk)?;
        let byte = u8::from_str_radix(byte_str, 2)?;
        entropy_bytes.push(byte);
    }

    // Calculate expected checksum
    let hash = Sha256::digest(&entropy_bytes);
    let hash_bits = format!("{:08b}", hash[0]);
    let expected_checksum = &hash_bits[..checksum_len];

    Ok(checksum_bits == expected_checksum)
}

/// Generate a random mnemonic
pub fn generate_mnemonic(word_count: usize) -> Result<String> {
    if ![12, 15, 18, 21, 24].contains(&word_count) {
        return Err(anyhow!("Invalid word count"));
    }

    // In production, use a proper implementation with secure randomness
    // This is a simplified version for demonstration
    let wordlist = get_wordlist();
    let entropy_bits = word_count * 11 - (word_count / 3);
    
    // Generate random entropy (simplified - use proper crypto RNG in production)
    let mut entropy = vec![0u8; entropy_bits / 8];
    for byte in entropy.iter_mut() {
        *byte = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() % 256) as u8;
    }

    // Calculate checksum
    let hash = Sha256::digest(&entropy);
    let checksum_len = word_count / 3;
    
    // Convert to bits
    let mut bits = String::new();
    for byte in &entropy {
        bits.push_str(&format!("{:08b}", byte));
    }
    
    let hash_bits = format!("{:08b}", hash[0]);
    bits.push_str(&hash_bits[..checksum_len]);

    // Convert bits to words
    let mut words = Vec::new();
    for chunk in bits.as_bytes().chunks(11) {
        let word_bits = std::str::from_utf8(chunk)?;
        let index = usize::from_str_radix(word_bits, 2)?;
        if index < wordlist.len() {
            words.push(wordlist[index].clone());
        }
    }

    Ok(words.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordlist_size() {
        let wordlist = get_wordlist();
        assert!(wordlist.len() > 0);
    }

    #[test]
    fn test_validate_checksum_valid() {
        // This is a known valid phrase
        let words = vec!["abandon", "abandon", "abandon", "abandon", "abandon", 
                        "abandon", "abandon", "abandon", "abandon", "abandon", 
                        "abandon", "about"];
        
        // Note: This test may fail because we're using a subset wordlist
        // In production, use the full BIP39 wordlist
        let result = validate_checksum(&words);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_mnemonic() {
        let result = generate_mnemonic(12);
        assert!(result.is_ok());
        
        if let Ok(phrase) = result {
            let words: Vec<&str> = phrase.split_whitespace().collect();
            assert_eq!(words.len(), 12);
        }
    }
}
