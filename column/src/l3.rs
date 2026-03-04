use super::*;
use shared::*;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering::Relaxed};

const LAYER1_WORDS: usize = BITSET_WORDS.div_ceil(64);

static COMP_PRESENCE: [BitSet; NUM_COMPS] = [const { BitSet::new() }; NUM_COMPS];
static COMP_VALUES: [CompColumn; NUM_COMPS] = [const { CompColumn::new() }; NUM_COMPS];
static ACTIVE_SLOTS: AtomicUsize = AtomicUsize::new(0);

static DEVICE_SCOPE: [BitSet; MAX_DEVICES] = [const { BitSet::new() }; MAX_DEVICES];
static GROUP_SCOPE: [BitSet; MAX_GROUPS] = [const { BitSet::new() }; MAX_GROUPS];

static DEVICE_RANGES: [AtomicU64; MAX_DEVICES] = [const { AtomicU64::new(0) }; MAX_DEVICES];
static GROUP_RANGE_DATA: [[AtomicU64; DEVICES_PER_GROUP]; MAX_GROUPS] =
    [const { [const { AtomicU64::new(0) }; DEVICES_PER_GROUP] }; MAX_GROUPS];
static GROUP_RANGE_LENS: [AtomicUsize; MAX_GROUPS] = [const { AtomicUsize::new(0) }; MAX_GROUPS];
static GROUP_DEVICE_DATA: [[AtomicU32; DEVICES_PER_GROUP]; MAX_GROUPS] =
    [const { [const { AtomicU32::new(0) }; DEVICES_PER_GROUP] }; MAX_GROUPS];
static GROUP_DEVICE_LENS: [AtomicUsize; MAX_GROUPS] = [const { AtomicUsize::new(0) }; MAX_GROUPS];

pub struct BitSet {
    layer0: [AtomicU64; BITSET_WORDS],
    layer1: [AtomicU64; LAYER1_WORDS],
    layer2: AtomicU64,
}

struct CompColumn {
    values: [AtomicU64; MAX_SLOTS],
}

pub fn init(device_count: usize) {
    let devices = test_devices(device_count);
    let groups = test_groups(device_count);
    let mut next_slot = 0;

    let mut device_slot_ranges: Vec<(_, _)> = Vec::with_capacity(device_count);

    for (device_idx, device) in devices.iter().enumerate() {
        let slot_start = next_slot;

        for entity in &device.entities {
            let slot = next_slot;
            next_slot += 1;

            DEVICE_SCOPE[device_idx].set_bit(slot);

            for comp in &entity.components {
                COMP_PRESENCE[comp.desc.index()].set_bit(slot);

                if let Some(value) = comp.value {
                    COMP_VALUES[comp.desc.index()].store(slot, value.to_u64());
                }
            }
        }

        let slot_end = next_slot;
        device_slot_ranges.push((slot_start, slot_end));
        DEVICE_RANGES[device_idx].store(pack_range(slot_start as u32, slot_end as u32), Relaxed);
    }

    for (group_idx, group) in groups.iter().enumerate() {
        for (i, &device_idx) in group.device_indices.iter().enumerate() {
            let (slot_start, slot_end) = device_slot_ranges[device_idx];

            for slot in slot_start..slot_end {
                GROUP_SCOPE[group_idx].set_bit(slot);
            }

            GROUP_RANGE_DATA[group_idx][i]
                .store(pack_range(slot_start as u32, slot_end as u32), Relaxed);
            GROUP_DEVICE_DATA[group_idx][i].store(device_idx as u32, Relaxed);
        }

        GROUP_RANGE_LENS[group_idx].store(group.device_indices.len(), Relaxed);
        GROUP_DEVICE_LENS[group_idx].store(group.device_indices.len(), Relaxed);
    }

    ACTIVE_SLOTS.store(next_slot, Relaxed);
}

#[inline(always)]
fn query_range_avg_f64(
    target_idx: usize,
    filter_idx: usize,
    start_slot: usize,
    end_slot: usize,
    sum: &mut f64,
    count: &mut usize,
) {
    if start_slot >= end_slot {
        return;
    }

    let start_word = start_slot / 64;
    let end_word = (end_slot - 1) / 64;

    for w in start_word..=end_word {
        let mut mask =
            COMP_PRESENCE[target_idx].load_word(w) & COMP_PRESENCE[filter_idx].load_word(w);

        if w == start_word {
            let start_bit = start_slot % 64;
            if start_bit != 0 {
                mask &= !((1 << start_bit) - 1);
            }
        }
        if w == end_word {
            let end_bit = end_slot % 64;
            if end_bit != 0 {
                mask &= (1 << end_bit) - 1;
            }
        }

        for_each_set_bit(mask, |bit| {
            let slot = w * 64 + bit;
            *sum += f64::from_bits(COMP_VALUES[target_idx].load(slot));
            *count += 1;
        });
    }
}

pub fn query_avg_f64(target: CompType, filter: CompType) -> (f64, usize) {
    let mut sum = 0.0;
    let mut count = 0;

    let bitsets = [
        &COMP_PRESENCE[target.index()],
        &COMP_PRESENCE[filter.index()],
    ];

    descend_3(&bitsets, |w, mask| {
        for_each_set_bit(mask, |bit| {
            let slot = w * 64 + bit;
            sum += f64::from_bits(COMP_VALUES[target.index()].load(slot));
            count += 1;
        });
    });

    finalize_avg(sum, count)
}

pub fn query_count(target: CompType, filter: CompType) -> usize {
    let mut count = 0;

    let bitsets = [
        &COMP_PRESENCE[target.index()],
        &COMP_PRESENCE[filter.index()],
    ];

    descend_3(&bitsets, |_w, mask| {
        count += mask.count_ones() as usize;
    });

    count
}

pub fn query_avg_f64_group_v1(target: CompType, filter: CompType, group: usize) -> (f64, usize) {
    let mut sum = 0.0;
    let mut count = 0;

    let bitsets = [
        &COMP_PRESENCE[target.index()],
        &COMP_PRESENCE[filter.index()],
        &GROUP_SCOPE[group],
    ];

    descend_3(&bitsets, |w, mask| {
        for_each_set_bit(mask, |bit| {
            let slot = w * 64 + bit;
            sum += f64::from_bits(COMP_VALUES[target.index()].load(slot));
            count += 1;
        });
    });

    finalize_avg(sum, count)
}

pub fn query_avg_f64_device_v1(target: CompType, filter: CompType, device: usize) -> (f64, usize) {
    let mut sum = 0.0;
    let mut count = 0;

    let bitsets = [
        &COMP_PRESENCE[target.index()],
        &COMP_PRESENCE[filter.index()],
        &DEVICE_SCOPE[device],
    ];

    descend_3(&bitsets, |w, mask| {
        for_each_set_bit(mask, |bit| {
            let slot = w * 64 + bit;
            sum += f64::from_bits(COMP_VALUES[target.index()].load(slot));
            count += 1;
        });
    });

    finalize_avg(sum, count)
}

pub fn query_avg_f64_device_v2(target: CompType, filter: CompType, device: usize) -> (f64, usize) {
    let (start, end) = load_device_range(device);
    let mut sum = 0.0;
    let mut count = 0;

    query_range_avg_f64(
        target.index(),
        filter.index(),
        start,
        end,
        &mut sum,
        &mut count,
    );

    finalize_avg(sum, count)
}

pub fn query_avg_f64_group_v2(target: CompType, filter: CompType, group: usize) -> (f64, usize) {
    let mut sum = 0.0;
    let mut count = 0;
    let len = GROUP_RANGE_LENS[group].load(Relaxed);

    for val in GROUP_RANGE_DATA[group].iter().take(len) {
        let (start, end) = unpack_range(val.load(Relaxed));
        query_range_avg_f64(
            target.index(),
            filter.index(),
            start,
            end,
            &mut sum,
            &mut count,
        );
    }

    finalize_avg(sum, count)
}

pub fn query_avg_f64_group_v3(target: CompType, filter: CompType, group: usize) -> (f64, usize) {
    let mut sum = 0.0;
    let mut count = 0;
    let len = GROUP_DEVICE_LENS[group].load(Relaxed);

    for val in GROUP_DEVICE_DATA[group].iter().take(len) {
        let device = val.load(Relaxed) as usize;
        let (start, end) = load_device_range(device);
        query_range_avg_f64(
            target.index(),
            filter.index(),
            start,
            end,
            &mut sum,
            &mut count,
        );
    }

    finalize_avg(sum, count)
}

impl BitSet {
    const fn new() -> Self {
        Self {
            layer0: [const { AtomicU64::new(0) }; BITSET_WORDS],
            layer1: [const { AtomicU64::new(0) }; LAYER1_WORDS],
            layer2: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    fn set_bit(&self, index: usize) {
        let word = index / 64;
        let bit = index % 64;
        self.layer0[word].fetch_or(1u64 << bit, Relaxed);

        let l1_word = word / 64;
        let l1_bit = word % 64;
        self.layer1[l1_word].fetch_or(1u64 << l1_bit, Relaxed);

        self.layer2.fetch_or(1u64 << l1_word, Relaxed);
    }

    fn clear_all(&self) {
        let mut l2 = self.layer2.load(Relaxed);
        while l2 != 0 {
            let l1_idx = l2.trailing_zeros() as usize;
            l2 &= l2 - 1;

            let mut l1 = self.layer1[l1_idx].load(Relaxed);
            self.layer1[l1_idx].store(0, Relaxed);

            while l1 != 0 {
                let w = l1_idx * 64 + l1.trailing_zeros() as usize;
                self.layer0[w].store(0, Relaxed);
                l1 &= l1 - 1;
            }
        }
        self.layer2.store(0, Relaxed);
    }

    #[inline(always)]
    fn load_word(&self, index: usize) -> u64 {
        self.layer0[index].load(Relaxed)
    }

    #[inline(always)]
    fn load_layer1(&self, index: usize) -> u64 {
        self.layer1[index].load(Relaxed)
    }

    #[inline(always)]
    fn load_layer2(&self) -> u64 {
        self.layer2.load(Relaxed)
    }
}

impl CompColumn {
    const fn new() -> Self {
        Self {
            values: [const { AtomicU64::new(0) }; MAX_SLOTS],
        }
    }

    #[inline(always)]
    fn store(&self, slot: usize, value: u64) {
        self.values[slot].store(value, Relaxed);
    }

    fn clear(&self, num_slots: usize) {
        self.values[..num_slots]
            .iter()
            .for_each(|val| val.store(0, Relaxed));
    }

    #[inline(always)]
    fn load(&self, slot: usize) -> u64 {
        self.values[slot].load(Relaxed)
    }
}

#[inline(always)]
fn pack_range(start: u32, end: u32) -> u64 {
    ((start as u64) << 32) | (end as u64)
}

#[inline(always)]
fn unpack_range(packed: u64) -> (usize, usize) {
    ((packed >> 32) as usize, (packed & 0xFFFFFFFF) as usize)
}

#[inline(always)]
fn load_device_range(device: usize) -> (usize, usize) {
    unpack_range(DEVICE_RANGES[device].load(Relaxed))
}

pub fn reset() {
    let slots = ACTIVE_SLOTS.load(Relaxed);
    if slots == 0 {
        return;
    }

    COMP_PRESENCE.iter().for_each(|bs| bs.clear_all());
    COMP_VALUES.iter().for_each(|col| col.clear(slots));

    DEVICE_SCOPE.iter().for_each(|bs| bs.clear_all());
    DEVICE_RANGES.iter().for_each(|r| r.store(0, Relaxed));

    GROUP_SCOPE.iter().for_each(|bs| bs.clear_all());
    GROUP_RANGE_LENS.iter().for_each(|len| len.store(0, Relaxed));
    GROUP_DEVICE_LENS.iter().for_each(|len| len.store(0, Relaxed));

    ACTIVE_SLOTS.store(0, Relaxed);
}

#[inline(always)]
fn for_each_set_bit(mut bits: u64, mut f: impl FnMut(usize)) {
    while bits != 0 {
        let bit = bits.trailing_zeros() as usize;
        f(bit);
        bits &= bits - 1;
    }
}

#[inline(always)]
fn finalize_avg(sum: f64, count: usize) -> (f64, usize) {
    if count == 0 {
        (0.0, 0)
    } else {
        (sum / count as f64, count)
    }
}

#[inline(always)]
fn descend_3<F>(bitsets: &[&BitSet], mut word_fn: F)
where
    F: FnMut(usize, u64),
{
    let mut l2 = bitsets
        .iter()
        .fold(u64::MAX, |acc, bs| acc & bs.load_layer2());

    while l2 != 0 {
        let l1_idx = l2.trailing_zeros() as usize;
        l2 &= l2 - 1;

        let mut l1 = bitsets
            .iter()
            .fold(u64::MAX, |acc, bs| acc & bs.load_layer1(l1_idx));

        while l1 != 0 {
            let w_offset = l1.trailing_zeros() as usize;
            l1 &= l1 - 1;

            let w = l1_idx * 64 + w_offset;
            let mask = bitsets
                .iter()
                .fold(u64::MAX, |acc, bs| acc & bs.load_word(w));

            if mask != 0 {
                word_fn(w, mask);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    const N: usize = 20;

    static INIT: Once = Once::new();
    fn ensure_init() {
        INIT.call_once(|| init(N));
    }

    #[test]
    fn avg_dimmer_with_light() {
        ensure_init();
        let (avg, count) = query_avg_f64(CompType::Dimmer, CompType::Light);
        let (expected_avg, expected_count) = expected_dimmer_with_light(N);
        assert_eq!(count, expected_count);
        assert!((avg - expected_avg).abs() < f64::EPSILON);
    }

    #[test]
    fn avg_real_with_sensor() {
        ensure_init();
        let (avg, count) = query_avg_f64(CompType::Real, CompType::Sensor);
        let (expected_avg, expected_count) = expected_real_with_sensor(N);
        assert_eq!(count, expected_count);
        assert!((avg - expected_avg).abs() < 1e-10);
    }

    #[test]
    fn no_match() {
        ensure_init();
        let count = query_count(CompType::Dimmer, CompType::Config);
        assert_eq!(count, 0);
    }

    #[test]
    fn slot_count() {
        ensure_init();
        assert_eq!(ACTIVE_SLOTS.load(Relaxed), total_slots(N));
    }

    #[test]
    fn v1_group_avg_dimmer() {
        ensure_init();
        let (avg, count) = query_avg_f64_group_v1(CompType::Dimmer, CompType::Light, 0);
        assert_eq!(count, 2);
        assert!((avg - (1.0 + 0.75) / 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn v1_device_rgb() {
        ensure_init();
        let (avg, count) = query_avg_f64_device_v1(CompType::Dimmer, CompType::Light, 0);
        assert_eq!(count, 1);
        assert!((avg - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn v2_device_rgb() {
        ensure_init();
        let (avg, count) = query_avg_f64_device_v2(CompType::Dimmer, CompType::Light, 0);
        assert_eq!(count, 1);
        assert!((avg - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn v2_group_avg_dimmer() {
        ensure_init();
        let (avg, count) = query_avg_f64_group_v2(CompType::Dimmer, CompType::Light, 0);
        assert_eq!(count, 2);
        assert!((avg - (1.0 + 0.75) / 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn v3_group_avg_dimmer() {
        ensure_init();
        let (avg, count) = query_avg_f64_group_v3(CompType::Dimmer, CompType::Light, 0);
        assert_eq!(count, 2);
        assert!((avg - (1.0 + 0.75) / 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn versions_agree_group() {
        ensure_init();
        let v1 = query_avg_f64_group_v1(CompType::Real, CompType::Sensor, 0);
        let v2 = query_avg_f64_group_v2(CompType::Real, CompType::Sensor, 0);
        let v3 = query_avg_f64_group_v3(CompType::Real, CompType::Sensor, 0);
        assert_eq!(v1.1, v2.1);
        assert_eq!(v1.1, v3.1);
        assert!((v1.0 - v2.0).abs() < 1e-10);
        assert!((v1.0 - v3.0).abs() < 1e-10);
    }

    #[test]
    fn versions_agree_device() {
        ensure_init();
        let v1 = query_avg_f64_device_v1(CompType::Real, CompType::Sensor, 0);
        let v2 = query_avg_f64_device_v2(CompType::Real, CompType::Sensor, 0);
        assert_eq!(v1.1, v2.1);
        assert!((v1.0 - v2.0).abs() < 1e-10);
    }
}
