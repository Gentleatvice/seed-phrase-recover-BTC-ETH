use anyhow::{anyhow, Result};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;

type HmacSha256 = Hmac<Sha256>;

/// Derive Bitcoin addresses from a seed phrase
pub fn derive_addresses(phrase: &str, count: usize) -> Result<Vec<String>> {
    let seed = mnemonic_to_seed(phrase, "")?;
    let mut addresses = Vec::new();

    for i in 0..count {
        // Simplified address derivation
        // In production, use proper BIP32/BIP44 implementation
        let address = derive_address_from_seed(&seed, i)?;
        addresses.push(address);
    }

    Ok(addresses)
}

/// Convert mnemonic to seed using PBKDF2
fn mnemonic_to_seed(mnemonic: &str, passphrase: &str) -> Result<Vec<u8>> {
    let salt = format!("mnemonic{}", passphrase);
    let mut seed = vec![0u8; 64];
    
    pbkdf2_hmac::<Sha256>(
        mnemonic.as_bytes(),
        salt.as_bytes(),
        2048,
        &mut seed
    );

    Ok(seed)
}

/// Derive a single address from seed (simplified)
fn derive_address_from_seed(seed: &[u8], index: usize) -> Result<String> {
    // This is a simplified implementation for demonstration
    // In production, use proper BIP32 hierarchical deterministic wallet derivation
    
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(&index.to_le_bytes());
    let hash = hasher.finalize();

    // Convert to Bitcoin address format (simplified)
    let address = format!("1{}", hex::encode(&hash[..20]));
    Ok(address)
}

/// Derive Ethereum address from seed phrase
pub fn derive_ethereum_address(phrase: &str, index: usize) -> Result<String> {
    let seed = mnemonic_to_seed(phrase, "")?;
    
    // Simplified Ethereum address derivation
    let mut hasher = Sha256::new();
    hasher.update(&seed);
    hasher.update(&index.to_le_bytes());
    let hash = hasher.finalize();

    // Ethereum address format (0x + 40 hex chars)
    let address = format!("0x{}", hex::encode(&hash[..20]));
    Ok(address)
}

/// Helper module for hex encoding
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    pub fn decode(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic_to_seed() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let result = mnemonic_to_seed(phrase, "");
        assert!(result.is_ok());
        
        if let Ok(seed) = result {
            assert_eq!(seed.len(), 64);
        }
    }

    #[test]
    fn test_derive_addresses() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let result = derive_addresses(phrase, 5);
        assert!(result.is_ok());
        
        if let Ok(addresses) = result {
            assert_eq!(addresses.len(), 5);
            for addr in addresses {
                assert!(addr.starts_with('1'));
            }
        }
    }

    #[test]
    fn test_hex_encoding() {
        let bytes = vec![0x12, 0x34, 0xab, 0xcd];
        let encoded = hex::encode(&bytes);
        assert_eq!(encoded, "1234abcd");
        
        let decoded = hex::decode(&encoded).unwrap();
        assert_eq!(decoded, bytes);
    }
}
