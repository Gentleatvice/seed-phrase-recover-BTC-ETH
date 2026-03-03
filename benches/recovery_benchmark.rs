use criterion::{black_box, criterion_group, criterion_main, Criterion};
use seed_recovery::{validate_phrase, suggest_words};

fn benchmark_phrase_validation(c: &mut Criterion) {
    let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    
    c.bench_function("validate_phrase", |b| {
        b.iter(|| validate_phrase(black_box(phrase)))
    });
}

fn benchmark_word_suggestions(c: &mut Criterion) {
    c.bench_function("suggest_words", |b| {
        b.iter(|| suggest_words(black_box("abandn"), black_box(2)))
    });
}

fn benchmark_levenshtein_distance(c: &mut Criterion) {
    use seed_recovery::validate_phrase;
    
    c.bench_function("levenshtein_distance", |b| {
        b.iter(|| {
            // This will internally use levenshtein for suggestions
            suggest_words(black_box("test"), black_box(2))
        })
    });
}

criterion_group!(
    benches,
    benchmark_phrase_validation,
    benchmark_word_suggestions,
    benchmark_levenshtein_distance
);
criterion_main!(benches);
