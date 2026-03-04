use shared::*;
use smallvec::SmallVec;

pub struct Presense(u32);

pub struct Entity {
    comps: SmallVec<[(CompType, u64); 8]>,
    indices: [u8; NUM_COMPS],
}

pub struct Device {
    presense: Presense,
    type_to_entity: [SmallVec<[usize; 4]>; NUM_COMPS],
    entities: SmallVec<[Entity; 16]>,
}

pub struct Group {
    device_indices: Vec<usize>,
}

pub struct DeviceTree {
    devices: Vec<Device>,
    groups: Vec<Group>,
}

pub fn query_avg_f64(tree: &DeviceTree, target: CompType, filter: CompType) -> (f64, usize) {
    let mut sum = 0.0_f64;
    let mut count = 0_usize;

    for device in &tree.devices {
        query_device_avg_f64_inner(device, target, filter, &mut sum, &mut count);
    }

    finalize_avg(sum, count)
}

pub fn query_avg_f64_group(
    tree: &DeviceTree,
    target: CompType,
    filter: CompType,
    group: usize,
) -> (f64, usize) {
    let mut sum = 0.0_f64;
    let mut count = 0_usize;

    for &didx in &tree.groups[group].device_indices {
        let device = &tree.devices[didx];
        query_device_avg_f64_inner(device, target, filter, &mut sum, &mut count);
    }

    finalize_avg(sum, count)
}

pub fn query_avg_f64_device(
    tree: &DeviceTree,
    target: CompType,
    filter: CompType,
    device_idx: usize,
) -> (f64, usize) {
    let mut sum = 0.0_f64;
    let mut count = 0_usize;

    let device = &tree.devices[device_idx];
    query_device_avg_f64_inner(device, target, filter, &mut sum, &mut count);

    finalize_avg(sum, count)
}

pub fn query_count(tree: &DeviceTree, target: CompType, filter: CompType) -> usize {
    let mut count = 0_usize;

    for device in &tree.devices {
        query_device_count_inner(device, target, filter, &mut count);
    }

    count
}

impl Presense {
    fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    fn set(&mut self, comp: CompType) {
        self.0 |= 1 << comp.index();
    }

    #[inline(always)]
    fn has(&self, comp: CompType) -> bool {
        (self.0 & (1 << comp.index())) != 0
    }
}

impl Entity {
    fn new() -> Self {
        Self {
            comps: SmallVec::new(),
            indices: [u8::MAX; NUM_COMPS],
        }
    }

    fn put(&mut self, comp: Comp) -> Option<CompType> {
        let idx = comp.desc.index();
        let existing = self.indices[idx];

        if existing == u8::MAX {
            let offset = self.comps.len() as u8;
            let value = comp.value.map(|v| v.to_u64()).unwrap_or(0);
            self.comps.push((comp.desc, value));
            self.indices[idx] = offset;
            Some(comp.desc)
        } else {
            let value = comp.value.map(|v| v.to_u64()).unwrap_or(0);
            self.comps[existing as usize] = (comp.desc, value);
            None
        }
    }

    #[inline(always)]
    fn has(&self, comp: CompType) -> bool {
        self.indices[comp.index()] != u8::MAX
    }

    #[inline(always)]
    fn get_value(&self, comp: CompType) -> Option<u64> {
        let idx = self.indices[comp.index()];
        if idx != u8::MAX {
            Some(self.comps[idx as usize].1)
        } else {
            None
        }
    }
}

impl Device {
    fn new() -> Self {
        Self {
            presense: Presense::new(),
            type_to_entity: [const { SmallVec::new_const() }; NUM_COMPS],
            entities: SmallVec::new(),
        }
    }
}

impl DeviceTree {
    pub fn new(device_count: usize) -> Self {
        let test_data = test_devices(device_count);
        let test_groups = test_groups(device_count);
        let mut devices = Vec::with_capacity(test_data.len());

        for test_device in &test_data {
            let mut device = Device::new();

            for test_entity in &test_device.entities {
                let entity_idx = device.entities.len();
                let mut entity = Entity::new();

                for comp in &test_entity.components {
                    if let Some(desc) = entity.put(*comp) {
                        device.presense.set(desc);
                        device.type_to_entity[desc.index()].push(entity_idx);
                    }
                }

                device.entities.push(entity);
            }

            devices.push(device);
        }

        let groups = test_groups
            .into_iter()
            .map(|g| Group {
                device_indices: g.device_indices,
            })
            .collect();

        Self { devices, groups }
    }
}

#[inline(always)]
fn query_device_avg_f64_inner(
    device: &Device,
    target: CompType,
    filter: CompType,
    sum: &mut f64,
    count: &mut usize,
) {
    if !device.presense.has(target) || !device.presense.has(filter) {
        return;
    }

    let entity_indices = &device.type_to_entity[filter.index()];

    for &eidx in entity_indices {
        let entity = &device.entities[eidx];

        if let Some(raw) = entity.get_value(target) {
            *sum += f64::from_bits(raw);
            *count += 1;
        }
    }
}

#[inline(always)]
fn query_device_count_inner(
    device: &Device,
    target: CompType,
    filter: CompType,
    count: &mut usize,
) {
    if !device.presense.has(target) || !device.presense.has(filter) {
        return;
    }

    let entity_indices = &device.type_to_entity[filter.index()];

    for &eidx in entity_indices {
        let entity = &device.entities[eidx];
        if entity.has(target) {
            *count += 1;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 20;

    #[test]
    fn avg_dimmer_with_light() {
        let tree = DeviceTree::new(N);
        let (avg, count) = query_avg_f64(&tree, CompType::Dimmer, CompType::Light);
        let (expected_avg, expected_count) = expected_dimmer_with_light(N);
        assert_eq!(count, expected_count);
        assert!((avg - expected_avg).abs() < f64::EPSILON);
    }

    #[test]
    fn avg_real_with_sensor() {
        let tree = DeviceTree::new(N);
        let (avg, count) = query_avg_f64(&tree, CompType::Real, CompType::Sensor);
        let (expected_avg, expected_count) = expected_real_with_sensor(N);
        assert_eq!(count, expected_count);
        assert!((avg - expected_avg).abs() < 1e-10);
    }

    #[test]
    fn group_avg_dimmer_with_light() {
        let tree = DeviceTree::new(N);
        let (avg, count) = query_avg_f64_group(&tree, CompType::Dimmer, CompType::Light, 0);
        // Group 0 has devices 0..5 (archetypes 0,1,2,3,4)
        // Only archetypes 0 (RGB, 1.0) and 1 (Dimmable, 0.75) have Light+Dimmer
        assert_eq!(count, 2);
        assert!((avg - (1.0 + 0.75) / 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn device_scoped_dimmer() {
        let tree = DeviceTree::new(N);
        // Device 0 is RGB Light, dimmer = 1.0
        let (avg, count) = query_avg_f64_device(&tree, CompType::Dimmer, CompType::Light, 0);
        assert_eq!(count, 1);
        assert!((avg - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn device_scoped_dimmer_dimmable() {
        let tree = DeviceTree::new(N);
        // Device 1 is Dimmable Light, dimmer = 0.75
        let (avg, count) = query_avg_f64_device(&tree, CompType::Dimmer, CompType::Light, 1);
        assert_eq!(count, 1);
        assert!((avg - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn device_scoped_no_dimmer() {
        let tree = DeviceTree::new(N);
        // Device 2 is Smart Plug, no Light+Dimmer
        let (avg, count) = query_avg_f64_device(&tree, CompType::Dimmer, CompType::Light, 2);
        assert_eq!(count, 0);
        assert_eq!(avg, 0.0);
    }

    #[test]
    fn no_match() {
        let tree = DeviceTree::new(N);
        let count = query_count(&tree, CompType::Dimmer, CompType::Config);
        assert_eq!(count, 0);
    }

    #[test]
    fn entity_count() {
        let tree = DeviceTree::new(N);
        let total: usize = tree.devices.iter().map(|d| d.entities.len()).sum();
        assert_eq!(total, total_slots(N));
    }
}
