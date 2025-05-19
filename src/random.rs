use std::{
    ops::Deref,
    rc::Rc,
    sync::Mutex,
    time::Instant };
use instant_hasher::hash;


const A: u64 = 48_271;
const M: u64 = 2_147_483_647;
fn next(current: u64) -> u64 { A.wrapping_mul(current) % M }

fn get_u64() -> u64 { hash(Instant::now()) }


pub fn choose_from<T>(vec: &Vec<Rc<T>>, rng: &mut Rng) -> Rc<T> {
    vec[rng.next_index(vec.len())].clone() }

pub fn default_rng() -> Rng { Rng::new() }

pub fn with_seed(seed: u64) -> Rng { Rng::from_seed(seed) }

pub fn test_rng() -> Rng { Rng::from_seed(1) }

pub fn new_id() -> u64 { get_u64() }


fn wrap_seed(seed: u64) -> Rc<Mutex<u64>> {
    Rc::new(Mutex::new(seed)) }


#[derive(Clone)]
pub struct Rng {
    seed: Rc<Mutex<u64>>
}
impl Rng {
    pub fn from_seed(seed: u64) -> Self {
        if seed == 0 { 
            return Self { seed: wrap_seed(1) } }
        Self { seed: wrap_seed(seed) } }

    pub fn new() -> Self {
        Self { seed: wrap_seed(get_u64()) } }

    pub fn next(&mut self) -> u64 {
        let mut seed = self.seed.lock().unwrap();
        let next = next(*seed.deref());
        *seed = next;
        next }

    pub fn next_index(&mut self, length: usize) -> usize {
        let base = (self.next() - 1) as usize;
        base % length }
}


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


#[cfg(test)]
mod tests {
    use crate::random::default_rng;

    #[test]
    fn test() {
        let mut rng = default_rng();
        let count = 1_000;
        let mut vals = Vec::with_capacity(count);
        let mut expected = Vec::with_capacity(count);
        for _ in 0..count {
            vals.push(false);
            expected.push(true);
        }
        for _ in 0..(count * 10) {
            let rand = rng.next_index(count);
            vals[rand] = true;
        }
        assert_eq!(vals, expected)
    }
}