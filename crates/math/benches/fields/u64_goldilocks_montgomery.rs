use std::hint::black_box;

use criterion::Criterion;
use lambdaworks_math::{
    field::{
        element::FieldElement,
        fields::{
            fft_friendly::u64_goldilocks::{
                MontgomeryConfigU64GoldilocksPrimeField, U64GoldilocksPrimeField,
            },
            montgomery_backed_prime_fields::IsModulus,
        },
    },
    unsigned_integer::{
        element::{UnsignedInteger, U64},
        montgomery::MontgomeryAlgorithms,
    },
};
use rand::{Rng, SeedableRng};

pub type F = FieldElement<U64GoldilocksPrimeField>;
const NUM_LIMBS: usize = 1;

pub fn rand_field_elements(num: usize) -> Vec<(F, F)> {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(9001);
    let mut result = Vec::with_capacity(num);
    for _ in 0..result.capacity() {
        let rand_a = UnsignedInteger {
            limbs: [rng.gen::<u64>()],
        };
        let rand_b = UnsignedInteger {
            limbs: [rng.gen::<u64>()],
        };
        result.push((F::new(rand_a), F::new(rand_b)));
    }
    result
}

pub fn u64_goldilocks_montgomery_ops_benchmarks(c: &mut Criterion) {
    let input: Vec<Vec<(F, F)>> = [1, 10, 100, 1000, 10000, 100000, 1000000]
        .into_iter()
        .map(rand_field_elements)
        .collect::<Vec<_>>();
    let mut group = c.benchmark_group("U64 Goldilocks Mont operations");

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("add {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, y) in i {
                    black_box(black_box(x) + black_box(y));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("mul {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, y) in i {
                    black_box(black_box(x) * black_box(y));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("pow by 1 {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, _) in i {
                    black_box(black_box(x).pow(1_u64));
                }
            });
        });
    }

    // The non-boxed constants are intentional as they are
    // normally computed at compile time.
    for i in input.clone().into_iter() {
        group.bench_with_input(format!("sos_square {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, _) in i {
                    MontgomeryAlgorithms::sos_square(
                        black_box(black_box(x.value())),
                        &<MontgomeryConfigU64GoldilocksPrimeField as IsModulus<U64>>::MODULUS,
                        &U64GoldilocksPrimeField::MU,
                    );
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("square {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, _) in i {
                    black_box(black_box(x).square());
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("square with pow {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, _) in i {
                    black_box(black_box(x).pow(2_u64));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("square with mul {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, _) in i {
                    black_box(black_box(x) * black_box(x));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(
            format!("pow {:?}", &i.len()),
            &(i, 5u64),
            |bench, (i, a)| {
                bench.iter(|| {
                    for (x, _) in i {
                        black_box(black_box(x).pow(*a));
                    }
                });
            },
        );
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("sub {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, y) in i {
                    black_box(black_box(x) - black_box(y));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("inv {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, _) in i {
                    black_box(black_box(x).inv().unwrap());
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("div {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, y) in i {
                    black_box(black_box(x) / black_box(y)).unwrap();
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("eq {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, y) in i {
                    black_box(black_box(x) == black_box(y));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("sqrt {:?}", &i.len()), &i, |bench, i| {
            bench.iter(|| {
                for (x, _) in i {
                    black_box(black_box(x).sqrt());
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("sqrt squared {:?}", &i.len()), &i, |bench, i| {
            let i: Vec<F> = i.iter().map(|(x, _)| x * x).collect();
            bench.iter(|| {
                for x in &i {
                    black_box(black_box(x).sqrt());
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("bitand {:?}", &i.len()), &i, |bench, i| {
            // Note: we should strive to have the number of limbs be generic... ideally this benchmark group itself should have a generic type that we call into from the main runner.
            let i: Vec<(UnsignedInteger<NUM_LIMBS>, UnsignedInteger<NUM_LIMBS>)> =
                i.iter().map(|(x, y)| (*x.value(), *y.value())).collect();
            bench.iter(|| {
                for (x, y) in &i {
                    black_box(black_box(*x) & black_box(*y));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("bitor {:?}", &i.len()), &i, |bench, i| {
            let i: Vec<(UnsignedInteger<NUM_LIMBS>, UnsignedInteger<NUM_LIMBS>)> =
                i.iter().map(|(x, y)| (*x.value(), *y.value())).collect();
            bench.iter(|| {
                for (x, y) in &i {
                    black_box(black_box(*x) | black_box(*y));
                }
            });
        });
    }

    for i in input.clone().into_iter() {
        group.bench_with_input(format!("bitxor {:?}", &i.len()), &i, |bench, i| {
            let i: Vec<(UnsignedInteger<NUM_LIMBS>, UnsignedInteger<NUM_LIMBS>)> =
                i.iter().map(|(x, y)| (*x.value(), *y.value())).collect();
            bench.iter(|| {
                for (x, y) in &i {
                    black_box(black_box(*x) ^ black_box(*y));
                }
            });
        });
    }
}
