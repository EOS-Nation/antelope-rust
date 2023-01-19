use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::symbol::symbol_code,
}
