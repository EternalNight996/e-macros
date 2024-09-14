use criterion::{black_box, criterion_group, criterion_main, Criterion};
use e_macros::value;

#[value]
#[derive(Debug, PartialEq)]
enum TestEnum {
    #[e(value = "variant_a", index = 1)]
    VariantA,
    #[e(value = "variant_b", index = 2)]
    VariantB(String),
    #[e(value = "variant_c", index = 3)]
    VariantC { field: i32 },
}

fn benchmark_to_string(c: &mut Criterion) {
    let variants = [
        TestEnum::VariantA,
        TestEnum::VariantB("test".to_string()),
        TestEnum::VariantC { field: 42 },
    ];
    c.bench_function("TestEnum to_string", |b| {
        b.iter(|| {
            for variant in &variants {
                black_box(variant.to_string());
            }
        })
    });
}

fn benchmark_from_str(c: &mut Criterion) {
    let values = ["variant_a", "variant_b", "variant_c"];
    c.bench_function("TestEnum from_str", |b| {
        b.iter(|| {
            for value in &values {
                let _ = black_box(TestEnum::try_from(*value));
            }
        })
    });
}

fn benchmark_index(c: &mut Criterion) {
    let variants = [
        TestEnum::VariantA,
        TestEnum::VariantB("test".to_string()),
        TestEnum::VariantC { field: 42 },
    ];
    c.bench_function("TestEnum index", |b| {
        b.iter(|| {
            for variant in &variants {
                black_box(variant.index());
            }
        })
    });
}

fn benchmark_from_index(c: &mut Criterion) {
    let indices = [1, 2, 3];
    c.bench_function("TestEnum from", |b| {
        b.iter(|| {
            for index in &indices {
                black_box(TestEnum::from(TestEnum::VariantC { field: *index }));
            }
        })
    });
}

fn benchmark_value(c: &mut Criterion) {
    let variants = [
        TestEnum::VariantA,
        TestEnum::VariantB("test".to_string()),
        TestEnum::VariantC { field: 42 },
    ];
    c.bench_function("TestEnum value", |b| {
        b.iter(|| {
            for variant in &variants {
                black_box(variant.value());
            }
        })
    });
}

fn benchmark_try_from_value(c: &mut Criterion) {
    let values = ["variant_a", "variant_b", "variant_c"];
    c.bench_function("TestEnum try_from", |b| {
        b.iter(|| {
            for value in &values {
                let _ = black_box(TestEnum::try_from(*value));
            }
        })
    });
}

fn benchmark_variant_count(c: &mut Criterion) {
    c.bench_function("TestEnum variant_count", |b| {
        b.iter(|| TestEnum::variant_count())
    });
}

criterion_group!(benches, 
    benchmark_to_string, 
    benchmark_from_str, 
    benchmark_index, 
    benchmark_from_index, 
    benchmark_value, 
    benchmark_try_from_value, 
    benchmark_variant_count
);
criterion_main!(benches);