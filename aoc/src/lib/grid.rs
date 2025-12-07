// Fast 2D grid implementation - to be a drop in replacement for HashMap
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

// unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//     ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
// }

const SIZE: u64 = 65535;

#[derive(Default)]
pub struct GridHasher(u64);

impl Hasher for GridHasher {
    fn write(&mut self, bytes: &[u8]) {
        // for chunk in bytes.chunks(8) {
        //     let val = u64::from_be_bytes(chunk.try_into().unwrap_or_default());
        //     self.0 = self.0.saturating_mul(SIZE);
        //     self.0 = self.0.saturating_add(val);
        // }
    }

    fn finish(&self) -> u64 {
        0 // self.0 as u64
    }
}

pub type GridBuildHasher = BuildHasherDefault<GridHasher>;

pub type Grid<K, V> = HashMap<K, V, GridBuildHasher>;
