use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rayon::prelude::IntoParallelRefIterator;
use simulation::entities::material::Material;
use simulation::sand_sim::Simulation;
use std::fs::File;
use std::io::{Read, Write};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const SIZE_BASE: usize = 32;
const SIZE_MAX_MUL: usize = 32;

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

fn bench_tick(c: &mut Criterion) {
    let fill_area = load_fill_area(SIZE_BASE, SIZE_MAX_MUL);

    c.bench_function("tick", |b| {
        let sim = setup_simulation(WIDTH, HEIGHT);

        sim.par_fill(&fill_area);

        b.iter(|| {
            sim.tick();
        });
    });
}

fn bench_par_tick(c: &mut Criterion) {
    let fill_area = load_fill_area(SIZE_BASE, SIZE_MAX_MUL);

    c.bench_function("par_tick", |b| {
        let sim = setup_simulation(WIDTH, HEIGHT);

        sim.par_fill(&fill_area);

        b.iter(|| {
            sim.par_tick();
        });
    });
}

fn bench_compare_tick(c: &mut Criterion) {
    let base: usize = SIZE_BASE;
    let max_mul: usize = SIZE_MAX_MUL;

    let fill_area = load_fill_area(base, max_mul);

    let mut group = c.benchmark_group("tick group");

    for i in 1..=max_mul {
        let current = base * i;
        group.bench_with_input(BenchmarkId::new("cmp_tick", current), &current, |b, i| {
            let sim = setup_simulation(current, current);

            sim.par_fill(&fill_area[0..(current ^ 2)]);

            b.iter(|| {
                sim.tick();
            });
        });

        group.bench_with_input(
            BenchmarkId::new("cmp_par_tick", current),
            &current,
            |b, i| {
                let sim = setup_simulation(current, current);

                sim.par_fill(&fill_area[0..(current ^ 2)]);

                b.iter(|| {
                    sim.par_tick();
                });
            },
        );
    }
    group.finish();
}

fn load_fill_area(base: usize, max_mul: usize) -> Vec<Material> {
    let filename = format!("fill_area_{base}_{max_mul}.dat");

    let mut fill_area_file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file {filename}: {e}.\nTrying to create file...");
            let mut file = File::create(&filename)
                .unwrap_or_else(|e| panic!("Failed to create file {filename}: {e}"));
            let fill_area = gen_area(base * max_mul, base * max_mul);

            let fill_area_u8: Vec<u8> = fill_area
                .iter()
                .map(|mat| -> u8 {
                    match mat {
                        Material::Sand => 0,
                        Material::SandGenerator => 1,
                        Material::Water => 2,
                        Material::WaterGenerator => 3,
                        Material::Air => 4,
                        Material::Fire => 5,
                        Material::Smoke => 6,
                        Material::Vapor => 7,
                        Material::Wood => 8,
                    }
                })
                .collect();

            file.write_all(&fill_area_u8).unwrap();
            file
        }
    };

    let mut fill_area_u8: Vec<u8> = vec![];

    fill_area_file
        .read_to_end(&mut fill_area_u8)
        .unwrap_or_else(|e| panic!("Failed to read fill_area from file {filename}: {e}"));

    let fill_area: Vec<Material> = fill_area_u8
        .iter()
        .map(|raw_mat| -> Material {
            match raw_mat {
                0 => Material::Sand,
                1 => Material::SandGenerator,
                2 => Material::Water,
                3 => Material::WaterGenerator,
                4 => Material::Air,
                5 => Material::Fire,
                6 => Material::Smoke,
                7 => Material::Vapor,
                8 => Material::Wood,
                _ => panic!("Unknown Material ID: {raw_mat}"),
            }
        })
        .collect();
    fill_area
}

criterion_group!(benches, bench_par_tick,);
criterion_main!(benches);
