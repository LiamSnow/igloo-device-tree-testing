use shared::ComponentDesc;
use std::hint::black_box;

const DEVICE_COUNT: usize = 30;
const ITERS: u64 = 100_000_000;

fn main() {
    column::l1::reset();
    column::l1::init(DEVICE_COUNT);

    for _ in 0..ITERS {
        black_box(column::l1::query_avg_f64(
            black_box(ComponentDesc::Dimmer),
            black_box(ComponentDesc::Light),
        ));
    }

    eprintln!("done");
}
