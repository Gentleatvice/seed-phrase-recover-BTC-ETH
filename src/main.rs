use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use seed_recovery::{RecoveryEngine, RecoveryOptions, WordlistLang};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "seed-recovery")]
#[command(about = "Seed Phrase Auto Recovery Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Recover a seed phrase with missing words
    Recover {
        /// Seed phrase with missing words (use ??? for unknown words)
        #[arg(short, long)]
        phrase: String,

        /// Target address to match
        #[arg(short, long)]
        address: Option<String>,

        /// Number of threads to use
        #[arg(short, long, default_value = "4")]
        threads: usize,

        /// Derivation path (default: m/44'/0'/0'/0/0)
        #[arg(short, long)]
        derivation: Option<String>,

        /// Cryptocurrency (bitcoin, ethereum)
        #[arg(short = 'c', long, default_value = "bitcoin")]
        crypto: String,
    },

    /// Validate a complete seed phrase
    Validate {
        /// Complete seed phrase to validate
        #[arg(short, long)]
        phrase: String,

        /// Show derived addresses
        #[arg(short, long)]
        addresses: bool,
    },

    /// Suggest similar words for a typo
    Suggest {
        /// Word with possible typo
        #[arg(short, long)]
        word: String,

        /// Maximum edit distance
        #[arg(short, long, default_value = "2")]
        max_distance: usize,
    },

    /// Generate a random valid seed phrase (for testing)
    Generate {
        /// Word count (12, 15, 18, 21, or 24)
        #[arg(short, long, default_value = "12")]
        words: usize,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Recover {
            phrase,
            address,
            threads,
            derivation,
            crypto,
        } => {
            println!("{}", "🔍 Starting recovery process...".bright_cyan().bold());
            println!("{} {}", "Input phrase:".bright_white(), phrase);
            
            if let Some(ref addr) = address {
                println!("{} {}", "Target address:".bright_white(), addr);
            }
            
            println!("{} {}", "Threads:".bright_white(), threads);
            println!();

            let options = RecoveryOptions {
                phrase: phrase.clone(),
                target_address: address,
                threads,
                derivation_path: derivation,
                crypto_type: crypto,
                lang: WordlistLang::English,
            };

            let engine = RecoveryEngine::new(options);
            
            match engine {
                Ok(mut engine) => {
                    let pb = ProgressBar::new_spinner();
                    pb.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} [{elapsed_precise}] {msg}")
                            .unwrap(),
                    );
                    pb.set_message("Recovering...");

                    match engine.recover() {
                        Ok(result) => {
                            pb.finish_and_clear();
                            println!("{}", "✅ Recovery successful!".bright_green().bold());
                            println!();
                            println!("{} {}", "Recovered phrase:".bright_white().bold(), result.phrase);
                            println!("{} {}", "Address:".bright_white(), result.address);
                            println!("{} {}", "Attempts:".bright_white(), result.attempts);
                            println!("{} {:?}", "Time taken:".bright_white(), result.duration);
                        }
                        Err(e) => {
                            pb.finish_and_clear();
                            eprintln!("{} {}", "❌ Recovery failed:".bright_red().bold(), e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "❌ Initialization error:".bright_red().bold(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Validate { phrase, addresses } => {
            println!("{}", "🔍 Validating seed phrase...".bright_cyan().bold());
            
            match seed_recovery::validate_phrase(&phrase) {
                Ok(valid) => {
                    if valid {
                        println!("{}", "✅ Phrase is valid!".bright_green().bold());
                        
                        if addresses {
                            println!();
                            println!("{}", "Derived addresses:".bright_white().bold());
                            
                            if let Ok(addrs) = seed_recovery::derive_addresses(&phrase, 5) {
                                for (i, addr) in addrs.iter().enumerate() {
                                    println!("  [{}] {}", i, addr);
                                }
                            }
                        }
                    } else {
                        println!("{}", "❌ Phrase is invalid!".bright_red().bold());
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "❌ Validation error:".bright_red().bold(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Suggest { word, max_distance } => {
            println!("{} '{}'", "🔍 Finding similar words for".bright_cyan(), word);
            
            match seed_recovery::suggest_words(&word, max_distance) {
                Ok(suggestions) => {
                    if suggestions.is_empty() {
                        println!("{}", "No similar words found.".yellow());
                    } else {
                        println!();
                        println!("{} {} suggestions:", "Found".bright_green(), suggestions.len());
                        for (i, suggestion) in suggestions.iter().enumerate().take(20) {
                            println!("  {}. {}", i + 1, suggestion);
                        }
                        if suggestions.len() > 20 {
                            println!("  ... and {} more", suggestions.len() - 20);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "❌ Error:".bright_red().bold(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Generate { words } => {
            println!("{} {} word phrase...", "🎲 Generating".bright_cyan(), words);
            
            match seed_recovery::generate_phrase(words) {
                Ok(phrase) => {
                    println!();
                    println!("{}", "Generated phrase:".bright_white().bold());
                    println!("{}", phrase.bright_yellow());
                    println!();
                    println!("{}", "⚠️  This is for TESTING only! Never use generated phrases for real funds.".bright_red());
                }
                Err(e) => {
                    eprintln!("{} {}", "❌ Generation error:".bright_red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
    }
}
