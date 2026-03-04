pub const MAX_SLOTS: usize = 2048;
pub const MAX_DEVICES: usize = 128;
pub const MAX_GROUPS: usize = 64;
pub const BITSET_WORDS: usize = MAX_SLOTS / 64;

pub mod l1;
pub mod l2;
pub mod l3;
