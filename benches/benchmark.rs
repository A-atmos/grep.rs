use criterion::{black_box,criterion_group, criterion_main, Criterion};

use grep_rs::{matcher,KMP,algorithms};


pub fn bencher(c: &mut Criterion){

    let test_str : String = String::from("exGN8FqFM0YinHEvfduM7rje8J7yRSTydBBc4n4YXHhcLqRxuPdzM0dFLSucOlfrwKVxol6PaWZ9szKKx3enQ9CDUZmPJmy1CsZM0NPm7LJLsBZjJDTy9awApsYIdsPq1jbFqF9UG6K2tVbqCE23ibTYhe2fZOTtSqOB6k1ZkZYL5wYToZonm5l8UlBlhDR2KhflygQ7flXZJJSqSYZM9m9Bl4SwLQl96MMOGZrXDt1Ruuu90rawGuQhwmITjLQwoE7HCfnH3TafN3EMhZEh6z9mVZJhmrAUddrZzbuUVUlyEB9mZkVR2OY02Scq7FshvrXgPYgU0VjRMDVkXDLBVGpVvdin41zQDrwFqn8LZiUL2wUPaRz06QOYzp5IHRuzphe8FgobEAzyw6KnD9Ni2D3JhuCqfWMvY1D8sDksa2BA6Nh6vCHoxnynVjlorYWHrFz8KYXEeSS9Ug7D35Uc4NyiMUvoQ8tHAJ14Q8TG1aONHv8URj5JUPU1QAGzOMhNwbez60ZZLMwykS7mNlINzii5aOCCSYBCtN4SRNIlDUYRlSo81HP11NmuGP1AkCZ1qZGqXbxnyutuHAdGclFsWdp6EOnzXNzUi4l2nnibMmg1gXPH3KUDUX8cSr2yQ7IbHPDQFG4q5W8V6XCfnntZWtquACNynONJjipunzX1LXePcOIx00AmDcR7GpCMdz0KedlQKByESuJoaakrJBJ3Y7oRMo0ghUXvGGoew3Y0Gh6hC7XiJbOe0UbOtadt4uk7OkR1YDjAJfcniQhjbGw9KUhehSunt8keYcDQOGTTYDIPoJX8xIkADhY74TYYdKUCU8DyFW7Sy0A1vZbi9pU65MSoevUf1WS91zl78FqbvtadUq0K8RrNoT3CbxwES3p9NCtn6jY19TM7YCp00Zza1dcH2hBoHGwNsUi0kqHATczpnatPzpFprHfGKRNNM5N5ziYj6TgxqrU4xa7WDWJmimAHZiUThEKQQMwdqFai");
    let test_str2 : String = String::from("uNyNRZ8MgCo8a5C3xdOYffZeXJSdDyhJTJgVlRViAnPUwtzLUyMEUcaTC7I1EVBWAbJftrEdjD28Qdw2SUQ72WEM20peNuPTFrZShziW8UCZqrDI7xkLvQypl1ryKRex1ikMTdI92b0RU9qTXO9iS6M48RWM61FAOXpLt5Qw1ccDEnZ2cT90BtpcxephMUPQVBc6mYjWaW5FxrvwjywsfwpEN8tzaFcN6pT0U6sseJIMfiQp14BkIdzNZpauVbIejgvKz5Y9vK46q9CpBbTz9G4zFjMejaAUtPrXCCTnTPjh49aUuQzfFQoGcUvgQtWqzMTO6UJVa83Lu9rZe9b5wgMPNydhGnKg5eebqSWRkwgYEaAScIJ6r2THXTbkfh4RULcTDB2DKOk0Yd0B616zPxD8ZNnwEpbx2p9CcpV8SPB7y7FlWwvFUQFwAbbpq05xVH9MEkdA56kFBpdVjkzfzwwlzSBXtoN6MJxPnNwtSGIMQAUOkfg91HjwyXlkpZcfi9gSDbwoflXeGUeakYPCCE4TyZWxBGdiPO8493MbXs5QcEH44nR21VTT9GwFg12S2srcN9LnwaZFCRnlukM8VCYdJSS4uZLDX950sFY9FlT4sseblvMDcK60iAy99DbAglZOyJWUAyTa6COy4lQmzc8yrY4KAT5SJOQ0Br4g45bZflm3PBtZObWTPaT20VgfIJiFegMZEVSIUP7Ec48RiN8AcbGT2u9ria5HJUvY9669fw9OiYVBt0XinBd5ppz408pTFAMXGNVEDfOS8VchAIvYVS7ZspR7NreBUqX1jMC7bfqSzPBMkXKp5zgiH8yMHkO8YMslqX5PNukTtygLgEpNmUL4HyVPapwIXtTlX2KyGBbGCBQZJJZaoY3kfwbmYncEWrocH3erZlYyt0ugQfz25CGtYCRT2rPE21BF25IABdt81Sl9rYMNCa7AXN0NQYBkzXOOI1SXrL9odYe6CTOBeQQMnUhvegXtsaR");
    // Benchmark test for KMP Algorithm
    c.bench_function("KMP Benchmark", |b| b.iter(
        || KMP::new(
            black_box(
                &test_str)
            ).index_of_any(black_box(&test_str2))));

    // Benchmark test for Regex Search using tabulation method
    c.bench_function("RegEx Benchmark", |b|
        b.iter(
            || matcher::is_match_regex(
                black_box(test_str.clone()), 
                black_box(test_str2.clone()+"*?")
            )
        )
    );

    c.bench_function("Rabin Karp Algo", |b|b.iter(
        || algorithms::rabin_karp(
            black_box(test_str.clone()), 
            black_box(test_str2.clone())
        )
    ));
    
    c.bench_function("Boyer Moore", |b|b.iter(
        || algorithms::boyer_moore_search(
            black_box(&test_str), 
            black_box(&test_str2)
        )
    ));


}

criterion_group!(benches, bencher);

criterion_main!(benches);
