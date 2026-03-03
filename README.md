# 🔐 Seed Phrase Auto Recovery

A powerful tool for recovering cryptocurrency seed phrases with missing or incorrect words. Built with Rust for performance and JavaScript for web interface.

## ⚠️ Security Warning

**NEVER use this tool with your actual seed phrases on an online machine!**
- Use only on air-gapped computers
- This tool is for educational and recovery purposes only
- Always verify recovered phrases on official wallet software

## 🌟 Features

- **Fast Recovery**: Rust-powered brute force engine
- **Multiple Algorithms**: Support for BIP39, BIP44, BIP84
- **Checksum Validation**: Automatic validation of seed phrase checksums
- **Web Interface**: User-friendly JavaScript/HTML interface
- **CLI Support**: Command-line interface for advanced users
- **Multiple Cryptocurrencies**: Bitcoin, Ethereum, and more
- **Pattern Matching**: Smart word suggestion based on partial matches

## 📋 Requirements

- Rust 1.70+ (for backend)
- Node.js 16+ (for web interface)
- Cargo (comes with Rust)

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/seed-phrase-auto-recovery.git
cd seed-phrase-auto-recovery

# Build Rust backend
cargo build --release

# Install web dependencies
cd web
npm install
```

### Usage

#### CLI Mode

```bash
# Recover a seed phrase with one missing word
cargo run --release -- recover \
  --phrase "word1 word2 word3 ??? word5 word6 word7 word8 word9 word10 word11 word12" \
  --address "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"

# Check if a complete phrase is valid
cargo run --release -- validate \
  --phrase "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Find similar words
cargo run --release -- suggest --word "abandn"
```

#### Web Interface

```bash
cd web
npm run dev
# Open http://localhost:3000 in your browser
```

## 🔧 Configuration

Create a `config.toml` file:

```toml
[recovery]
# Number of threads to use
threads = 8

# Maximum attempts before stopping
max_attempts = 1000000

# Derivation path
derivation_path = "m/44'/0'/0'/0/0"

[blockchain]
# Blockchain to check against
network = "bitcoin"
```

## 📖 How It Works

1. **Input**: User provides a seed phrase with missing/incorrect words
2. **Validation**: System checks phrase length and structure
3. **Generation**: Rust engine generates possible combinations
4. **Verification**: Each combination is validated against BIP39 checksum
5. **Derivation**: Valid phrases are used to derive addresses
6. **Matching**: Derived addresses are compared with target address

## 🏗️ Architecture

```
seed-phrase-auto-recovery/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library exports
│   ├── recovery.rs        # Recovery engine
│   ├── bip39.rs           # BIP39 implementation
│   └── crypto.rs          # Cryptographic functions
├── web/                   # Web interface
│   ├── src/
│   │   ├── index.html     # Main HTML
│   │   ├── app.js         # JavaScript logic
│   │   └── styles.css     # Styling
│   └── package.json
├── tests/                 # Unit tests
├── benches/              # Performance benchmarks
├── Cargo.toml            # Rust dependencies
└── README.md
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run benchmarks
cargo bench

# Test with coverage
cargo tarpaulin --out Html
```

## 🔒 Security Best Practices

1. **Air-Gap**: Use only on offline computers
2. **No Storage**: Never save seed phrases to disk
3. **Memory Cleanup**: Tool overwrites sensitive data in memory
4. **Verification**: Always verify on official wallet software
5. **Audit**: Review source code before use

## 📊 Performance

Typical recovery times (on modern CPU):
- 1 missing word: < 1 second
- 2 missing words: 1-5 minutes
- 3 missing words: 2-8 hours
- 4 missing words: Days to weeks

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚖️ Legal Disclaimer

This tool is provided "as is" for educational and legitimate recovery purposes only. Users are responsible for:
- Ensuring legal right to recover the seed phrase
- Compliance with local laws and regulations
- Any consequences of using this tool

## 🙏 Acknowledgments

- [BIP39 Specification](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin)
- [bip39 crate](https://crates.io/crates/bip39)

## 📧 Contact

- Issues: [GitHub Issues](https://github.com/yourusername/seed-phrase-auto-recovery/issues)
- Discussions: [GitHub Discussions](https://github.com/yourusername/seed-phrase-auto-recovery/discussions)

---

**⭐ If this project helped you, please give it a star!**
