pub mod test;
pub use test::*;

pub const NUM_COMPS: usize = 16;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompType {
    Sensor = 0,
    Diagnostic = 1,
    Light = 2,
    Config = 3,
    DeviceClass = 4,
    Boolean = 5,
    Color = 6,
    Dimmer = 7,
    Switch = 8,
    ColorTemperature = 9,
    ColorMode = 10,
    SensorStateClass = 11,
    Icon = 12,
    Unit = 13,
    AccuracyDecimals = 14,
    Real = 15,
}

#[derive(Debug, Clone, Copy)]
pub enum CompValue {
    Bool(bool),
    F64(f64),
    U16(u16),
    Color { r: u8, g: u8, b: u8 },
    DeviceClass(DeviceClass),
    ColorMode(ColorModeKind),
    SensorStateClass(SensorStateClass),
    Icon(IconId),
    Unit(UnitId),
}

#[derive(Debug, Clone, Copy)]
pub struct Comp {
    pub desc: CompType,
    pub value: Option<CompValue>,
}

pub const ALL_COMPS: [CompType; NUM_COMPS] = [
    CompType::Sensor,
    CompType::Diagnostic,
    CompType::Light,
    CompType::Config,
    CompType::DeviceClass,
    CompType::Boolean,
    CompType::Color,
    CompType::Dimmer,
    CompType::Switch,
    CompType::ColorTemperature,
    CompType::ColorMode,
    CompType::SensorStateClass,
    CompType::Icon,
    CompType::Unit,
    CompType::AccuracyDecimals,
    CompType::Real,
];

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Connectivity = 0,
    Duration = 1,
    SignalStrength = 2,
    Restart = 3,
    Temperature = 4,
    Humidity = 5,
    Motion = 6,
    Door = 7,
    Power = 8,
    Energy = 9,
    Illuminance = 10,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModeKind {
    Temperature = 0,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorStateClass {
    TotalIncreasing = 0,
    Measurement = 1,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconId {
    TimerOutline = 0,
    RestartAlert = 1,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitId {
    Seconds = 0,
    DecibelsMilliwatt = 1,
    Watts = 2,
    KilowattHours = 3,
    Celsius = 4,
    Percent = 5,
    Lux = 6,
}

impl CompType {
    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn is_marker(self) -> bool {
        (self as u16) < 4
    }
}

impl CompValue {
    pub fn to_u64(self) -> u64 {
        match self {
            Self::Bool(b) => b as u64,
            Self::F64(f) => f.to_bits(),
            Self::U16(v) => v as u64,
            Self::Color { r, g, b } => ((r as u64) << 16) | ((g as u64) << 8) | (b as u64),
            Self::DeviceClass(v) => v as u64,
            Self::ColorMode(v) => v as u64,
            Self::SensorStateClass(v) => v as u64,
            Self::Icon(v) => v as u64,
            Self::Unit(v) => v as u64,
        }
    }
}

impl Comp {
    pub const fn marker(desc: CompType) -> Self {
        Self { desc, value: None }
    }

    pub fn valued(desc: CompType, value: CompValue) -> Self {
        Self {
            desc,
            value: Some(value),
        }
    }
}
