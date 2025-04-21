use std::rc::Rc;


const A: u64 = 48_271;
const M: u64 = 2_147_483_647;
fn next(current: u64) -> u64 { A.wrapping_mul(current) % M }


pub fn choose_from<T>(vec: &Vec<Rc<T>>, rng: &mut Rng) -> Rc<T> { 
    vec[rng.next_index(vec.len())].clone() }

pub fn default_rng() -> Rng { Rng::new() }

pub fn with_seed(seed: u64) -> Rng { Rng::from_seed(seed) }

pub fn test_rng() -> Rng { Rng::from_seed(1) }


#[derive(Copy, Clone)]
pub struct Rng {
    seed: u64
}
impl Rng {
    pub fn from_seed(seed: u64) -> Self {
        if seed == 0 { 
            return Self { seed: 1 } }
        Self { seed } }
    
    pub fn new() -> Self {
        Self { seed: crate::u64_gen::get_u64() }
    }
    
    pub fn next(&mut self) -> u64 {
        self.seed = next(self.seed);
        self.seed }
    
    pub fn next_index(&mut self, length: usize) -> usize {
        let base = (self.next() - 1) as usize;
        base % length }
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