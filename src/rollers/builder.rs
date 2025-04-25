use std::{
    ops::{Add, Sub},
    rc::Rc };
use crate::{
    {Unit, Value},
    rollers::{SubRoller, ModifierRoller, PoolRoller, MathRoller} };

struct Builder(Rc<dyn SubRoller>);
impl Builder {
    pub fn finish(self) -> Rc<dyn SubRoller> { self.0 }
    pub fn n_times(self, n: u8) -> Builder { Builder(PoolRoller::basic(n, self.0)) }
    pub fn pool(self, n: u8, strategy: crate::rollers::Strategy) -> Builder { 
        Builder(PoolRoller::new(n, self.0, strategy).unwrap()) }
    pub fn with_modifier(self, relationship: Rc<dyn Unit>, value: i32) -> Builder {
        Builder(ModifierRoller::new(self.0, Value { unit: relationship, value })) }
}
impl Add<Rc<dyn SubRoller>> for Builder {
    type Output = Self;
    
    fn add(self, rhs: Rc<dyn SubRoller>) -> Builder {
        Builder(MathRoller::plus(self.0, rhs)) }
}
impl Add<Value> for Builder {
    type Output = Self;
    
    fn add(self, rhs: Value) -> Builder {
        Builder(ModifierRoller::new(self.0, rhs)) }
}
impl Sub<Rc<dyn SubRoller>> for Builder {
    type Output = Self;
    
    fn sub(self, rhs: Rc<dyn SubRoller>) -> Builder {
        Builder(SubtractRoller::new(self.0, rhs)) }
}
impl Sub<Value> for Builder {
    type Output = Self;
    
    fn sub(self, rhs: Value) -> Builder {
        let value = Value{ unit: rhs.unit, value: rhs.value * -1 };
        Builder(ModifierRoller::new(self.0, value)) }
}