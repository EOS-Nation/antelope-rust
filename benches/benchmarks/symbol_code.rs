use antelope::symbol_code::SymbolCode;
use criterion::{black_box, criterion_group, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let symcode = SymbolCode::new("FOO");

    c.bench_function("SymbolCode::From<&str>", |b| {
        b.iter(|| SymbolCode::from(black_box("FOO")))
    });
    c.bench_function("symcode.is_valid()", |b| b.iter(|| symcode.is_valid()));
    c.bench_function("!symcode", |b| b.iter(|| !symcode));
}

criterion_group!(symbol_code, criterion_benchmark);
