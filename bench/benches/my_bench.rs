use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use shared::CompType;

const SIZES: &[usize] = &[10, 20, 30, 40, 50, 100];

fn init_all(n: usize) -> old::DeviceTree {
    let old_tree = old::DeviceTree::new(n);
    column::l1::reset();
    column::l1::init(n);
    column::l2::reset();
    column::l2::init(n);
    column::l3::reset();
    column::l3::init(n);
    old_tree
}

macro_rules! bench_variants {
    ($c:expr, $name:expr, $old_tree:ident, [$(($label:expr, $query:expr)),+ $(,)?]) => {{
        let mut group = $c.benchmark_group($name);
        for &n in SIZES {
            let $old_tree = init_all(n);
            $(
                group.bench_with_input(BenchmarkId::new($label, n), &n, |b, _| {
                    b.iter(|| $query)
                });
            )+
        }
        group.finish();
    }};
}

fn global_avg_dimmer_with_light(c: &mut Criterion) {
    bench_variants!(
        c,
        "global_avg_dimmer_with_light",
        old_tree,
        [
            (
                "old",
                old::query_avg_f64(&old_tree, CompType::Dimmer, CompType::Light)
            ),
            (
                "l1",
                column::l1::query_avg_f64(CompType::Dimmer, CompType::Light)
            ),
            (
                "l2",
                column::l2::query_avg_f64(CompType::Dimmer, CompType::Light)
            ),
            (
                "l3",
                column::l3::query_avg_f64(CompType::Dimmer, CompType::Light)
            ),
        ]
    );
}

fn global_avg_real_with_sensor(c: &mut Criterion) {
    bench_variants!(
        c,
        "global_avg_real_with_sensor",
        old_tree,
        [
            (
                "old",
                old::query_avg_f64(&old_tree, CompType::Real, CompType::Sensor)
            ),
            (
                "l1",
                column::l1::query_avg_f64(CompType::Real, CompType::Sensor)
            ),
            (
                "l2",
                column::l2::query_avg_f64(CompType::Real, CompType::Sensor)
            ),
            (
                "l3",
                column::l3::query_avg_f64(CompType::Real, CompType::Sensor)
            ),
        ]
    );
}

fn global_count_no_match(c: &mut Criterion) {
    bench_variants!(
        c,
        "global_count_no_match",
        old_tree,
        [
            (
                "old",
                old::query_count(&old_tree, CompType::Dimmer, CompType::Config)
            ),
            (
                "l1",
                column::l1::query_count(CompType::Dimmer, CompType::Config)
            ),
            (
                "l2",
                column::l2::query_count(CompType::Dimmer, CompType::Config)
            ),
            (
                "l3",
                column::l3::query_count(CompType::Dimmer, CompType::Config)
            ),
        ]
    );
}

fn group_avg_dimmer_with_light(c: &mut Criterion) {
    bench_variants!(
        c,
        "group_avg_dimmer_with_light",
        old_tree,
        [
            (
                "old",
                old::query_avg_f64_group(&old_tree, CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l1_v1",
                column::l1::query_avg_f64_group_v1(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l2_v1",
                column::l2::query_avg_f64_group_v1(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l3_v1",
                column::l3::query_avg_f64_group_v1(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l1_v2",
                column::l1::query_avg_f64_group_v2(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l2_v2",
                column::l2::query_avg_f64_group_v2(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l3_v2",
                column::l3::query_avg_f64_group_v2(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l1_v3",
                column::l1::query_avg_f64_group_v3(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l2_v3",
                column::l2::query_avg_f64_group_v3(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l3_v3",
                column::l3::query_avg_f64_group_v3(CompType::Dimmer, CompType::Light, 0)
            ),
        ]
    );
}

fn group_avg_real_with_sensor(c: &mut Criterion) {
    bench_variants!(
        c,
        "group_avg_real_with_sensor",
        old_tree,
        [
            (
                "old",
                old::query_avg_f64_group(&old_tree, CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l1_v1",
                column::l1::query_avg_f64_group_v1(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l2_v1",
                column::l2::query_avg_f64_group_v1(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l3_v1",
                column::l3::query_avg_f64_group_v1(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l1_v2",
                column::l1::query_avg_f64_group_v2(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l2_v2",
                column::l2::query_avg_f64_group_v2(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l3_v2",
                column::l3::query_avg_f64_group_v2(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l1_v3",
                column::l1::query_avg_f64_group_v3(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l2_v3",
                column::l2::query_avg_f64_group_v3(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l3_v3",
                column::l3::query_avg_f64_group_v3(CompType::Real, CompType::Sensor, 0)
            ),
        ]
    );
}

fn device_avg_dimmer_with_light(c: &mut Criterion) {
    bench_variants!(
        c,
        "device_avg_dimmer_with_light",
        old_tree,
        [
            (
                "old",
                old::query_avg_f64_device(&old_tree, CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l1_v1",
                column::l1::query_avg_f64_device_v1(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l2_v1",
                column::l2::query_avg_f64_device_v1(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l3_v1",
                column::l3::query_avg_f64_device_v1(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l1_v2",
                column::l1::query_avg_f64_device_v2(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l2_v2",
                column::l2::query_avg_f64_device_v2(CompType::Dimmer, CompType::Light, 0)
            ),
            (
                "l3_v2",
                column::l3::query_avg_f64_device_v2(CompType::Dimmer, CompType::Light, 0)
            ),
        ]
    );
}

fn device_avg_real_with_sensor(c: &mut Criterion) {
    bench_variants!(
        c,
        "device_avg_real_with_sensor",
        old_tree,
        [
            (
                "old",
                old::query_avg_f64_device(&old_tree, CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l1_v1",
                column::l1::query_avg_f64_device_v1(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l2_v1",
                column::l2::query_avg_f64_device_v1(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l3_v1",
                column::l3::query_avg_f64_device_v1(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l1_v2",
                column::l1::query_avg_f64_device_v2(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l2_v2",
                column::l2::query_avg_f64_device_v2(CompType::Real, CompType::Sensor, 0)
            ),
            (
                "l3_v2",
                column::l3::query_avg_f64_device_v2(CompType::Real, CompType::Sensor, 0)
            ),
        ]
    );
}

criterion_group!(
    benches,
    global_avg_dimmer_with_light,
    global_avg_real_with_sensor,
    global_count_no_match,
    group_avg_dimmer_with_light,
    group_avg_real_with_sensor,
    device_avg_dimmer_with_light,
    device_avg_real_with_sensor,
);
criterion_main!(benches);
