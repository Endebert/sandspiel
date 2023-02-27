use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use simulation::entities::material::Material;
use simulation::sand_sim::Simulation;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

fn setup_simulation(width: usize, height: usize) -> Simulation {
    Simulation::new(width, height)
}

fn gen_area(width: usize, height: usize) -> Vec<Material> {
    let materials = [
        Material::Air,
        Material::Water,
        Material::Fire,
        Material::Sand,
        Material::Vapor,
        Material::Smoke,
        Material::Wood,
    ];

    let size = width * height;

    let mut rng = thread_rng();

    let mut fill_area = Vec::with_capacity(size);
    for _ in 0..size {
        fill_area.push(materials.choose(&mut rng).unwrap().clone());
    }

    fill_area
}

fn bench_compare_tick(c: &mut Criterion) {
    let base: usize = 32;
    let max_mul: usize = 32;

    let fill_area = gen_area(base * max_mul, base * max_mul);

    let mut group = c.benchmark_group("tick group");

    for i in 1..=max_mul {
        let current = base * i;
        group.bench_with_input(BenchmarkId::new("tick", current), &current, |b, i| {
            let sim = setup_simulation(current, current);

            sim.par_fill(&fill_area[0..(current ^ 2)]);

            b.iter(|| {
                sim.tick();
            });
        });

        group.bench_with_input(BenchmarkId::new("par_tick", current), &current, |b, i| {
            let sim = setup_simulation(current, current);

            sim.par_fill(&fill_area[0..(current ^ 2)]);

            b.iter(|| {
                sim.par_tick();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_compare_tick,);
criterion_main!(benches);
