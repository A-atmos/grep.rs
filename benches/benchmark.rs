use criterion::{black_box,criterion_group, criterion_main, Criterion};

use grep_rs::{matcher,KMP};


pub fn bencher(c: &mut Criterion){

    // Benchmark test for KMP Algorithm
    c.bench_function("KMP Benchmark", |b| b.iter(
        || KMP::new(
            black_box(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
            ).index_of_any(black_box("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"))));

    // Benchmark test for Regex Search using tabulation method
    c.bench_function("RegEx Benchmark", |b|
        b.iter(
            || matcher::is_match_regex(
                black_box(String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")), 
                black_box(String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789*?"))
            )
        )
    );

    

}

criterion_group!(benches, bencher);

criterion_main!(benches);
