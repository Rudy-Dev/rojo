use std::path::{Path, PathBuf};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use tempfile::{tempdir, TempDir};

use librojo::commands::{build, BuildOptions};

pub fn benchmark_small_place(c: &mut Criterion) {
    bench_build_place(c, "Small Place", "test-projects/benchmark_small_place")
}

criterion_group!(benches, benchmark_small_place);
criterion_main!(benches);

fn bench_build_place(c: &mut Criterion, name: &str, path: &str) {
    let mut group = c.benchmark_group(name);

    // 'rojo build' generally takes a fair bit of time to execute.
    group.sample_size(10);
    group.bench_function("build", |b| {
        b.iter_batched(
            || place_setup(path),
            |(_dir, options)| build(&options).unwrap(),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn place_setup<P: AsRef<Path>>(input_path: P) -> (TempDir, BuildOptions) {
    let dir = tempdir().unwrap();
    let input = input_path.as_ref().to_path_buf();
    let output_file = dir.path().join("output.rbxlx");

    let options = BuildOptions {
        fuzzy_project_path: input,
        output_file,
        output_kind: None,
    };

    (dir, options)
}
