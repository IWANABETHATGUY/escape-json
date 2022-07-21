use criterion::{black_box, criterion_group, criterion_main, Criterion};
use escape_json::{escape_json, two_pass_replace, two_pass_search_one_pass_copy, regex_replace, regex_iter_replace};

fn criterion_benchmark(c: &mut Criterion) {
    test!("array.json", c);
    test!("big.json", c);
    test!("escape.json", c);
    test!("large.json", c);
    test!("package.json", c);
    test!("escape_heavy.json", c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[macro_export]
macro_rules! test {
    ($filename:expr, $c:ident) => {{
        let source = include_str!(concat!("../assets/", $filename));
        $c.bench_function(concat!($filename, " @`two_pass_replace`"), |b| {
            b.iter(|| two_pass_replace(black_box(source)))
        });
        $c.bench_function(concat!($filename, " @`escape_json`"), |b| {
            b.iter(|| escape_json(black_box(source)))
        });
        $c.bench_function(concat!($filename, " @`two_pass_search_one_pass_copy`"), |b| {
            b.iter(|| two_pass_search_one_pass_copy(black_box(source)))
        });

        $c.bench_function(concat!($filename, " @`regex`"), |b| {
            b.iter(|| regex_replace(black_box(source)))
        });

        $c.bench_function(concat!($filename, " @`regex_iter`"), |b| {
            b.iter(|| regex_iter_replace(black_box(source)))
        });
    }};
}
