use criterion::{criterion_group, criterion_main, Criterion};
use musahih_pro::core::language_detector::LanguageDetector;

fn benchmark_detector(c: &mut Criterion) {
    let detector = LanguageDetector::default();
    c.bench_function("analyze_common_wrong_layout_word", |b| {
        b.iter(|| detector.analyze("ghbdk"))
    });
}

criterion_group!(benches, benchmark_detector);
criterion_main!(benches);
