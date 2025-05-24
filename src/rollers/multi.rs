use std::rc::Rc;
use crate::{
    random::Rng,
    rollers::{Roll, Roller} };


#[derive(Clone)]
pub struct NamedRoller {
  name: String,
  roller: Rc<dyn Roller>
}
impl NamedRoller {
    pub fn new(name: String, roller: Rc<dyn Roller>) -> Self {
        Self { name, roller } }
    
    pub fn numbered(idx: usize, roller: Rc<dyn Roller>) -> Self {
        Self { name: (idx + 1).to_string(), roller } }
    
    fn description(&self) -> String {
        format!("{}: {}", self.name, self.roller.description()) }
    
    fn roll_with(&self, rng: Rng) -> NamedRoll {
        NamedRoll { name: self.name.clone(), roll: self.roller.clone().roll_with(rng) } }
}


pub struct MultiRoller {
    inner: Vec<NamedRoller>
}
impl MultiRoller {
    pub fn new(inner: Vec<NamedRoller>) -> Rc<Self> { Rc::new(MultiRoller{ inner }) }
    
    pub fn add(&mut self, roller: NamedRoller) -> &Self {
        self.inner.push(roller);
        self }
    
    pub fn add_with_name(&mut self, name: String, roller: Rc<dyn Roller>) -> &Self {
        self.inner.push(NamedRoller::new(name, roller));
        self }
    
    pub fn add_numbered(&mut self, roller: Rc<dyn Roller>) -> &Self {
        self.inner.push(NamedRoller::numbered(self.inner.len(), roller));
        self }
}
impl Roller for MultiRoller {
    fn description(&self) -> String {
        self.inner.iter()
          .map(|roller| roller.description())
          .collect::<Vec<String>>()
          .join("\n") }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        self.inner.iter()
          .map(|roller| roller.roll_with(rng.clone()))
          .collect::<MultiRoll>()
          .boxed() }
}


pub struct NamedRoll {
    name: String,
    roll: Box<dyn Roll>
}
impl NamedRoll {
    fn intermediate_results(&self) -> String {
        format!("{}: {}", self.name, self.roll.intermediate_results()) }

    fn final_result(&self) -> String {
        format!("{}: {}", self.name, self.roll.final_result()) }
}


struct MultiRoll {
    inner: Vec<NamedRoll>
}
impl MultiRoll {
    fn boxed(self) -> Box<Self> { Box::new(self) }
}
impl Roll for MultiRoll {
    fn intermediate_results(&self) -> String {
        self.inner.iter()
          .map(NamedRoll::intermediate_results)
          .collect::<Vec<String>>()
          .join("\n") }
    
    fn final_result(&self) -> String {
        self.inner.iter()
          .map(NamedRoll::final_result)
          .collect::<Vec<String>>()
          .join("\n") }
}
impl FromIterator<NamedRoll> for MultiRoll {
    fn from_iter<T: IntoIterator<Item=NamedRoll>>(iter: T) -> Self {
        MultiRoll { inner: iter.into_iter().collect() } }
}