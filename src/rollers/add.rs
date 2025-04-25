use std::{
    rc::Rc
};

use crate::{
    random::Rng, 
    rollers::{Roller, Roll, SubRoller, SubRoll}};

enum MathRollerType {
    First(Rc<dyn SubRoller>),
    Add(Rc<dyn SubRoller>),
    Subtract(Rc<dyn SubRoller>)
}
use MathRollerType::*;

pub struct MathRoller {
    inner: Vec<MathRollerType>
}
impl MathRoller {
    pub fn add(lhs: Rc<dyn SubRoller>, rhs: Rc<dyn SubRoller>) -> Rc<Self> {
        Rc::new(Self{ inner: vec![First(lhs), Add(rhs)] }) }
    
    pub fn plus(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(Add(roller));
        self }
    
    pub fn plus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(Add));
        self }
    
    pub fn minus(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(Subtract(roller));
        self }
    
    pub fn minus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(Subtract));
        self }
}
impl Roller for MathRoller {
    fn description(&self) -> String {
        ; }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        ; }
}
impl SubRoller for MathRoller {
    fn is_simple(&self) -> bool { false }

    fn sub_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        todo!()
    }
}