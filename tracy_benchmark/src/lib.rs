use std::{hint, time};

// TODO: Remove outliers.
pub fn bench<F>(id: &str, mut f: F)
where
    F: FnMut(),
{
    println!("---");
    println!("id: {id}");
    let warmup_secs = time::Duration::from_secs(3);
    println!("warm up: {:?}", warmup_secs);

    let mut counter = 0;
    let warmup_start = time::Instant::now();

    while warmup_start.elapsed() < warmup_secs {
        hint::black_box(f());
        counter += 1;
    }

    let each = warmup_secs / counter;
    let target_s = time::Duration::from_millis(20);

    let s = 100;
    let mut i = (target_s.as_secs_f64() / each.as_secs_f64()).round() as u64;

    if i == 0 {
        i = 1;
    }

    println!("sampling: {s}");
    println!("iterations per sample: {i}");
    let mut samples = Vec::new();

    for _ in 0..s {
        let start = time::Instant::now();

        for _ in 0..i {
            hint::black_box(f());
        }

        let end = start.elapsed();
        samples.push(end);
    }

    let mean = samples
        .iter()
        .map(|s| s.as_secs_f64() / i as f64)
        .sum::<f64>()
        / s as f64;

    let formatted_mean = match mean {
        s if s >= 1.0 => format!("{:.3} s", s),
        s if s >= 1e-3 => format!("{:.3} ms", s * 1e3),
        s if s >= 1e-6 => format!("{:.3} µs", s * 1e6),
        s if s >= 1e-9 => format!("{:.3} ns", s * 1e9),
        s => format!("{:.3} ps", s * 1e12),
    };

    println!("mean: {}", formatted_mean);
}
