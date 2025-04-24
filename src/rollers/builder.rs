use std::{
    ops::{Add, Sub},
    rc::Rc };
use crate::{
    {Relationship, Value},
    rollers::{ComposableRoller, ModifierRoller, PoolRoller} };

struct Builder(Rc<dyn ComposableRoller>);
impl Builder {
    pub fn finish(self) -> Rc<dyn ComposableRoller> { self.0 }
    pub fn n_times(self, n: u8) -> Builder { Builder(PoolRoller::basic(n, self.0)) }
    pub fn pool(self, n: u8, strategy: crate::rollers::Strategy) -> Builder { 
        Builder(PoolRoller::new(n, self.0, strategy).unwrap()) }
    pub fn with_modifier(self, relationship: Rc<dyn Relationship>, value: i32) -> Builder {
        Builder(ModifierRoller::new(self.0, Value { relationship, value })) }
}
impl Add<Rc<dyn ComposableRoller>> for Builder {
    type Output = Self;
    
    fn add(self, rhs: Rc<dyn ComposableRoller>) -> Builder {
        Builder(AddRoller::new(self.0, rhs)) }
}
impl Add<Value> for Builder {
    type Output = Self;
    
    fn add(self, rhs: Value) -> Builder {
        Builder(ModifierRoller::new(self.0, rhs)) }
}
impl Sub<Rc<dyn ComposableRoller>> for Builder {
    type Output = Self;
    
    fn sub(self, rhs: Rc<dyn ComposableRoller>) -> Builder {
        Builder(SubtractRoller::new(self.0, rhs)) }
}
impl Sub<Value> for Builder {
    type Output = Self;
    
    fn sub(self, rhs: Value) -> Builder {
        let value = Value{ relationship: rhs.relationship, value: rhs.value * -1 };
        Builder(ModifierRoller::new(self.0, value)) }
}