use shared::CompType;
use std::hint::black_box;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle, sleep};
use std::time::{Duration, Instant};

const NUM_THREADS: usize = 18;
const DURATION_SECS: u64 = 3;
const WARMUP_SECS: u64 = 1;
const CHECK_INTERVAL: u64 = 1024;

fn main() {
    println!("Igloo L2 V1 Throughput");
    println!("{NUM_THREADS} threads, {WARMUP_SECS}s warmup, {DURATION_SECS}s per run");
    println!("{}", "=".repeat(100));

    let device_counts = [30, 100];

    for &n in &device_counts {
        println!("\n--- {n} devices ---\n");

        bench_throughput("global avg Dimmer w/ Light", n, global_dimmer_light);
        bench_throughput("global avg Real w/ Sensor", n, global_real_sensor);
        bench_throughput("group avg Dimmer w/ Light", n, group_dimmer_light);
        bench_throughput("group avg Real w/ Sensor", n, group_real_sensor);
        bench_throughput("device avg Dimmer w/ Light", n, device_dimmer_light);
        bench_throughput("device avg Real w/ Sensor", n, device_real_sensor);
    }
}
fn run_threads(stop: &Arc<AtomicBool>, query_fn: fn() -> (f64, usize)) -> Vec<JoinHandle<u64>> {
    (0..NUM_THREADS)
        .map(|_| {
            let stop = Arc::clone(stop);
            thread::spawn(move || {
                let mut count = 0;
                loop {
                    for _ in 0..CHECK_INTERVAL {
                        black_box(query_fn());
                        count += 1;
                    }
                    if stop.load(Ordering::Relaxed) {
                        break;
                    }
                }
                count
            })
        })
        .collect()
}

fn bench_throughput(label: &str, device_count: usize, query_fn: fn() -> (f64, usize)) {
    column::l2::reset();
    column::l2::init(device_count);

    let stop = Arc::new(AtomicBool::new(false));
    let handles = run_threads(&stop, query_fn);
    sleep(Duration::from_secs(WARMUP_SECS));
    stop.store(true, Ordering::Relaxed);
    for h in handles {
        h.join().unwrap();
    }

    let stop = Arc::new(AtomicBool::new(false));
    let handles = run_threads(&stop, query_fn);
    let start = Instant::now();
    thread::sleep(Duration::from_secs(DURATION_SECS));
    stop.store(true, Ordering::Relaxed);

    let total: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    let elapsed = start.elapsed().as_secs_f64();
    let throughput = total as f64 / elapsed;

    println!(
        "{label:<45} | {device_count:>3} devices | {throughput:>12.0} queries/sec | {total:>12} total in {elapsed:.2}s",
    );
}

fn global_dimmer_light() -> (f64, usize) {
    column::l2::query_avg_f64(CompType::Dimmer, CompType::Light)
}

fn global_real_sensor() -> (f64, usize) {
    column::l2::query_avg_f64(CompType::Real, CompType::Sensor)
}

fn group_dimmer_light() -> (f64, usize) {
    column::l2::query_avg_f64_group_v1(CompType::Dimmer, CompType::Light, 0)
}

fn group_real_sensor() -> (f64, usize) {
    column::l2::query_avg_f64_group_v1(CompType::Real, CompType::Sensor, 0)
}

fn device_dimmer_light() -> (f64, usize) {
    column::l2::query_avg_f64_device_v1(CompType::Dimmer, CompType::Light, 0)
}

fn device_real_sensor() -> (f64, usize) {
    column::l2::query_avg_f64_device_v1(CompType::Real, CompType::Sensor, 0)
}
