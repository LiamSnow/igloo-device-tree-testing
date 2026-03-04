//! some AI generated homes lol

use super::*;

pub const NUM_ARCHETYPES: usize = 6;
pub const DEVICES_PER_GROUP: usize = 5;

pub struct TestEntity {
    pub name: String,
    pub components: Vec<Comp>,
}

pub struct TestDevice {
    pub name: String,
    pub entities: Vec<TestEntity>,
}

pub struct TestGroup {
    pub name: String,
    pub device_indices: Vec<usize>,
}

const GROUP_NAMES: &[&str] = &[
    "Living Room",
    "Kitchen",
    "Bedroom",
    "Office",
    "Bathroom",
    "Garage",
    "Hallway",
    "Dining Room",
    "Basement",
    "Attic",
    "Patio",
    "Guest Room",
    "Nursery",
];

fn rgb_light(index: usize) -> TestDevice {
    TestDevice {
        name: format!("RGB Light {index}"),
        entities: vec![
            TestEntity {
                name: "Status".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Connectivity),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(true)),
                ],
            },
            TestEntity {
                name: "RGBCT Bulb".into(),
                components: vec![
                    Comp::marker(CompType::Light),
                    Comp::valued(
                        CompType::Color,
                        CompValue::Color {
                            r: 255,
                            g: 255,
                            b: 255,
                        },
                    ),
                    Comp::valued(CompType::Dimmer, CompValue::F64(1.0)),
                    Comp::valued(CompType::Switch, CompValue::Bool(true)),
                    Comp::valued(CompType::ColorTemperature, CompValue::U16(2000)),
                    Comp::valued(
                        CompType::ColorMode,
                        CompValue::ColorMode(ColorModeKind::Temperature),
                    ),
                ],
            },
            TestEntity {
                name: "Uptime".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::TotalIncreasing),
                    ),
                    Comp::valued(CompType::Icon, CompValue::Icon(IconId::TimerOutline)),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Duration),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::Seconds)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(54.019)),
                ],
            },
            TestEntity {
                name: "WiFi Signal".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::SignalStrength),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::DecibelsMilliwatt)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(-57.0)),
                ],
            },
            TestEntity {
                name: "Reset".into(),
                components: vec![
                    Comp::marker(CompType::Config),
                    Comp::valued(CompType::Icon, CompValue::Icon(IconId::RestartAlert)),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Restart),
                    ),
                ],
            },
            TestEntity {
                name: "Safe Mode".into(),
                components: vec![
                    Comp::marker(CompType::Config),
                    Comp::valued(CompType::Icon, CompValue::Icon(IconId::RestartAlert)),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Restart),
                    ),
                ],
            },
        ],
    }
}

fn dimmable_light(index: usize) -> TestDevice {
    TestDevice {
        name: format!("Dimmable Light {index}"),
        entities: vec![
            TestEntity {
                name: "Status".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Connectivity),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(true)),
                ],
            },
            TestEntity {
                name: "Bulb".into(),
                components: vec![
                    Comp::marker(CompType::Light),
                    Comp::valued(CompType::Dimmer, CompValue::F64(0.75)),
                    Comp::valued(CompType::Switch, CompValue::Bool(true)),
                ],
            },
            TestEntity {
                name: "Uptime".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::TotalIncreasing),
                    ),
                    Comp::valued(CompType::Icon, CompValue::Icon(IconId::TimerOutline)),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Duration),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::Seconds)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(120.5)),
                ],
            },
            TestEntity {
                name: "WiFi Signal".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::SignalStrength),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::DecibelsMilliwatt)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(-45.0)),
                ],
            },
            TestEntity {
                name: "Reset".into(),
                components: vec![
                    Comp::marker(CompType::Config),
                    Comp::valued(CompType::Icon, CompValue::Icon(IconId::RestartAlert)),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Restart),
                    ),
                ],
            },
        ],
    }
}

fn smart_plug(index: usize) -> TestDevice {
    TestDevice {
        name: format!("Smart Plug {index}"),
        entities: vec![
            TestEntity {
                name: "Status".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Connectivity),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(true)),
                ],
            },
            TestEntity {
                name: "Relay".into(),
                components: vec![Comp::valued(CompType::Switch, CompValue::Bool(true))],
            },
            TestEntity {
                name: "Power".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Power),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::Watts)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(1)),
                    Comp::valued(CompType::Real, CompValue::F64(150.3)),
                ],
            },
            TestEntity {
                name: "Energy".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::TotalIncreasing),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Energy),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::KilowattHours)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(2)),
                    Comp::valued(CompType::Real, CompValue::F64(87.2)),
                ],
            },
            TestEntity {
                name: "WiFi Signal".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::SignalStrength),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::DecibelsMilliwatt)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(-62.0)),
                ],
            },
        ],
    }
}

fn motion_sensor(index: usize) -> TestDevice {
    TestDevice {
        name: format!("Motion Sensor {index}"),
        entities: vec![
            TestEntity {
                name: "Status".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Connectivity),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(true)),
                ],
            },
            TestEntity {
                name: "Motion".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Motion),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(false)),
                ],
            },
            TestEntity {
                name: "Illuminance".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Illuminance),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::Lux)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(342.7)),
                ],
            },
            TestEntity {
                name: "WiFi Signal".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::SignalStrength),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::DecibelsMilliwatt)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(-51.0)),
                ],
            },
        ],
    }
}

fn climate_sensor(index: usize) -> TestDevice {
    TestDevice {
        name: format!("Climate Sensor {index}"),
        entities: vec![
            TestEntity {
                name: "Status".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Connectivity),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(true)),
                ],
            },
            TestEntity {
                name: "Temperature".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Temperature),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::Celsius)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(1)),
                    Comp::valued(CompType::Real, CompValue::F64(22.5)),
                ],
            },
            TestEntity {
                name: "Humidity".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Humidity),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::Percent)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(55.0)),
                ],
            },
            TestEntity {
                name: "WiFi Signal".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::SignalStrength),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::DecibelsMilliwatt)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(-48.0)),
                ],
            },
            TestEntity {
                name: "Reset".into(),
                components: vec![
                    Comp::marker(CompType::Config),
                    Comp::valued(CompType::Icon, CompValue::Icon(IconId::RestartAlert)),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Restart),
                    ),
                ],
            },
        ],
    }
}

fn door_sensor(index: usize) -> TestDevice {
    TestDevice {
        name: format!("Door Sensor {index}"),
        entities: vec![
            TestEntity {
                name: "Status".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Connectivity),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(true)),
                ],
            },
            TestEntity {
                name: "Contact".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Door),
                    ),
                    Comp::valued(CompType::Boolean, CompValue::Bool(false)),
                ],
            },
            TestEntity {
                name: "WiFi Signal".into(),
                components: vec![
                    Comp::marker(CompType::Sensor),
                    Comp::marker(CompType::Diagnostic),
                    Comp::valued(
                        CompType::SensorStateClass,
                        CompValue::SensorStateClass(SensorStateClass::Measurement),
                    ),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::SignalStrength),
                    ),
                    Comp::valued(CompType::Unit, CompValue::Unit(UnitId::DecibelsMilliwatt)),
                    Comp::valued(CompType::AccuracyDecimals, CompValue::U16(0)),
                    Comp::valued(CompType::Real, CompValue::F64(-70.0)),
                ],
            },
            TestEntity {
                name: "Reset".into(),
                components: vec![
                    Comp::marker(CompType::Config),
                    Comp::valued(CompType::Icon, CompValue::Icon(IconId::RestartAlert)),
                    Comp::valued(
                        CompType::DeviceClass,
                        CompValue::DeviceClass(DeviceClass::Restart),
                    ),
                ],
            },
        ],
    }
}

pub fn test_devices(count: usize) -> Vec<TestDevice> {
    (0..count)
        .map(|i| match i % NUM_ARCHETYPES {
            0 => rgb_light(i),
            1 => dimmable_light(i),
            2 => smart_plug(i),
            3 => motion_sensor(i),
            4 => climate_sensor(i),
            5 => door_sensor(i),
            _ => unreachable!(),
        })
        .collect()
}

pub fn test_groups(device_count: usize) -> Vec<TestGroup> {
    let group_count = device_count.div_ceil(DEVICES_PER_GROUP);

    (0..group_count)
        .map(|g| {
            let start = g * DEVICES_PER_GROUP;
            let end = (start + DEVICES_PER_GROUP).min(device_count);
            TestGroup {
                name: GROUP_NAMES[g % GROUP_NAMES.len()].to_string(),
                device_indices: (start..end).collect(),
            }
        })
        .collect()
}

/// Number of entities for the device at the given index.
pub fn entities_per_device(device_index: usize) -> usize {
    match device_index % NUM_ARCHETYPES {
        0 => 6, // RGB Light
        1 => 5, // Dimmable Light
        2 => 5, // Smart Plug
        3 => 4, // Motion Sensor
        4 => 5, // Climate Sensor
        5 => 4, // Door Sensor
        _ => unreachable!(),
    }
}

/// Total entity (slot) count for a given number of devices.
pub fn total_slots(device_count: usize) -> usize {
    (0..device_count).map(entities_per_device).sum()
}

// ---- Expected Results ----
//
// Per-archetype Dimmer+Light:
//   0 (RGB):      1 match, dimmer = 1.0
//   1 (Dimmable): 1 match, dimmer = 0.75
//   2..5:         0 matches
//
// Per-archetype Sensor+Real (entities where BOTH Sensor and Real are present):
//   0 (RGB):      Uptime(54.019) + WiFi(-57.0)           = 2 matches
//   1 (Dimmable): Uptime(120.5)  + WiFi(-45.0)           = 2 matches
//   2 (Plug):     Power(150.3) + Energy(87.2) + WiFi(-62.0) = 3 matches
//   3 (Motion):   Illuminance(342.7) + WiFi(-51.0)       = 2 matches
//   4 (Climate):  Temp(22.5) + Humidity(55.0) + WiFi(-48.0) = 3 matches
//   5 (Door):     WiFi(-70.0)                             = 1 match

const DIMMER_LIGHT_VALUES: [f64; NUM_ARCHETYPES] = [1.0, 0.75, 0.0, 0.0, 0.0, 0.0];
const DIMMER_LIGHT_COUNTS: [usize; NUM_ARCHETYPES] = [1, 1, 0, 0, 0, 0];

const REAL_SENSOR_SUMS: [f64; NUM_ARCHETYPES] = [
    54.019 + -57.0,       // arch 0
    120.5 + -45.0,        // arch 1
    150.3 + 87.2 + -62.0, // arch 2
    342.7 + -51.0,        // arch 3
    22.5 + 55.0 + -48.0,  // arch 4
    -70.0,                // arch 5
];
const REAL_SENSOR_COUNTS: [usize; NUM_ARCHETYPES] = [2, 2, 3, 2, 3, 1];

pub fn expected_dimmer_with_light(device_count: usize) -> (f64, usize) {
    let mut sum = 0.0f64;
    let mut count = 0usize;
    for i in 0..device_count {
        let arch = i % NUM_ARCHETYPES;
        sum += DIMMER_LIGHT_VALUES[arch];
        count += DIMMER_LIGHT_COUNTS[arch];
    }
    let avg = if count > 0 { sum / count as f64 } else { 0.0 };
    (avg, count)
}

pub fn expected_real_with_sensor(device_count: usize) -> (f64, usize) {
    let mut sum = 0.0f64;
    let mut count = 0usize;
    for i in 0..device_count {
        let arch = i % NUM_ARCHETYPES;
        sum += REAL_SENSOR_SUMS[arch];
        count += REAL_SENSOR_COUNTS[arch];
    }
    let avg = if count > 0 { sum / count as f64 } else { 0.0 };
    (avg, count)
}
