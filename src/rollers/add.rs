use std::{
    rc::Rc
};

use crate::{
    random::Rng, 
    rollers::{Roller, Roll, SubRoller, SubRoll, DieRoll}};

pub struct AddRoller {
    inner: Vec<Rc<dyn SubRoller>>
}
impl AddRoller {
    pub fn new(lhs: Rc<dyn SubRoller>, rhs: Rc<dyn SubRoller>) -> Rc<Self> {
        Rc::new(Self{ inner: vec![lhs, rhs] }) }
    
    pub fn add(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(roller);
        self }
    
    pub fn add_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers);
        self }
}
impl Roller for AddRoller {
    fn description(&self) -> String {
        ; }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        ; }
}
impl SubRoller for AddRoller {
    fn is_simple(&self) -> bool { false }

    fn sub_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        todo!()
    }
}