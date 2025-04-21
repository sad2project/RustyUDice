mod instant_hasher {
    use std::time::Instant;
    use std::hash::{Hash, Hasher};

    pub fn hash(instant: Instant) -> u64 {
        let mut hasher = InstantToNum::new();
        instant.hash(&mut hasher);
        hasher.finish()
    }


    struct InstantToNum {
        value: Option<u64>,
    }
    impl InstantToNum {
        fn new()-> Self { InstantToNum { value: None } }
    }
    impl Hasher for InstantToNum {
        fn finish(&self) -> u64 { self.value.unwrap() }
        fn write(&mut self, bytes: &[u8]) {
            let mut num: i64 = 0;
            if bytes.len() == 1 {
                num = i8::from_ne_bytes(bytes.try_into().unwrap()) as i64; }
            else if bytes.len() == 2 {
                num = i16::from_ne_bytes(bytes.try_into().unwrap()) as i64; }
            else if bytes.len() == 4 {
                num = i32::from_ne_bytes(bytes.try_into().unwrap()) as i64; }
            else if bytes.len() == 8 {
                num = i64::from_ne_bytes(bytes.try_into().unwrap()); }
            else if bytes.len() == 16 { 
                num = i128::from_ne_bytes(bytes.try_into().unwrap()) as i64; }
            self.write_i64(num);
        }
        fn write_i64(&mut self, val: i64) {
            self.value = match self.value {
                Some(old_val) => { Some(old_val + (val as u64)) }
                None => { Some((val as u64) << 32) } }
        }
    }
}

use instant_hasher::hash;
use std::time::Instant;

pub(crate) fn get_u64() -> u64 {
    hash(Instant::now())
}