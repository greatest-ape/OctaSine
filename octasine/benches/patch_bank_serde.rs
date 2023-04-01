use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

use octasine::sync::PatchBank;

fn create_bank() -> PatchBank {
    fastrand::seed(123);

    let bank = PatchBank::default();

    for patch in bank.patches.iter() {
        for p in patch.parameters.values() {
            p.set_value(fastrand::f32());
        }
    }

    bank
}

fn export_plain(c: &mut Criterion) {
    let bank = create_bank();

    c.bench_with_input(
        BenchmarkId::new("export_plain", "randomized"),
        &bank,
        |b, bank| {
            b.iter(|| bank.export_plain_bytes());
        },
    );
}

fn export_fxb(c: &mut Criterion) {
    let bank = create_bank();

    c.bench_with_input(
        BenchmarkId::new("export_fxb", "randomized"),
        &bank,
        |b, bank| {
            b.iter(|| bank.export_fxb_bytes());
        },
    );
}

fn import_plain(c: &mut Criterion) {
    let data = create_bank().export_plain_bytes();

    c.bench_with_input(
        BenchmarkId::new("import_plain", "randomized"),
        &data,
        |b, data| {
            b.iter(|| PatchBank::new_from_bytes(&data));
        },
    );
}

fn import_fxb(c: &mut Criterion) {
    let data = create_bank().export_fxb_bytes();

    c.bench_with_input(
        BenchmarkId::new("import_fxb", "randomized"),
        &data,
        |b, data| {
            b.iter(|| PatchBank::new_from_bytes(&data));
        },
    );
}

criterion_group!(benches, export_plain, export_fxb, import_plain, import_fxb);
criterion_main!(benches);
