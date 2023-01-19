use antelope::symbol::SymbolCode;
use criterion::{black_box, criterion_group, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let symcode = SymbolCode::from("FOO");

    c.bench_function("SymbolCode::From<&str>", |b| {
        b.iter(|| SymbolCode::from(black_box("FOO")))
    });
    c.bench_function("symcode.is_valid()", |b| b.iter(|| symcode.is_valid()));
}

criterion_group!(symbol_code, criterion_benchmark);
