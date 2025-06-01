use std::rc::Rc;
use crate::{
    Name, 
    random::Rng,
    rollers::{Roll, Roller} };


/// `NamedRoller` is designed specifically for `MultiRoller`, though there's no good reason you can't
/// use it separately. It simply wraps a roller and gives it a name. There are a few constructors,
/// `new()` being the obvious default one. But the other two are built from numbers. `numbered()` takes
/// a `usize` and just uses the string of the number, but `for_index()` takes an index and adds one
/// to turn it from 0-based to 1-based.
///
/// Note that, unlike on most rollers, the constructors of `NamedRoller` don't automatically wrap
/// themselves in `Rc`. The `rc()` method allows for a nice, simple way to do so quickly.
#[derive(Clone)]
pub struct NamedRoller {
  name: Name,
  roller: Rc<dyn Roller>
}
impl NamedRoller {
    pub fn new(name: Name, roller: Rc<dyn Roller>) -> Self {
        Self { name, roller } }
    
    pub fn for_index(idx: usize, roller: Rc<dyn Roller>) -> Self {
        Self { name: (idx + 1).into(), roller } }
        
    pub fn numbered(num: usize, roller: Rc<dyn Roller>) -> Self {
        Self { name: num.into(), roller } }
    
    pub fn rc(self) -> Rc<Self> {
        Rc::new(self) }
}
impl Roller for NamedRoller {
    fn description(&self) -> String {
        format!("{}: {}", self.name, self.roller.description()) }
    
    fn roll_with(&self, rng: Rng) -> Box<dyn Roll> {
        Box::new(NamedRoll { name: self.name.clone(), roll: self.roller.clone().roll_with(rng) }) }
}


/// `MultiRoller` is meant to run multiple rollers at once, but their results aren't combined; they'results
/// simply listed in order. This is useful for something like rolling attack and damage at once.
/// 
/// Each roll needs a label to distinguish them, so it get wrapped in a `NamedRoller`. This doesn't
/// doesn't need to be done explicitly, as `MultiRoller` can be created with an empty `Vec`, and you
/// can add the rollers with the `add*()` methods. If you don't want to come up with a name, just use
/// `add_numbered()` and it will be assigned a number instead, that number being the index it will
/// be stored at plus one (it's a 1-based count). Use `add()` if you've already wrapped it in a 
/// `NamedRoller`, and use `add_with_name()` to add the roller and a name, allowing `MultiRoller` to
/// do the wrapping for you.
/// 
/// Using `new_numbered()`, you can also build one from a collection of other rollers, and they will simply all get 
/// index-based names. 
pub struct MultiRoller {
    inner: Vec<NamedRoller>
}
impl MultiRoller {
    pub fn new(inner: Vec<NamedRoller>) -> Rc<Self> { Rc::new(MultiRoller{ inner }) }
    
    pub fn new_numbered(rollers: impl IntoIterator<Item=Rc<dyn Roller>>) -> Rc<Self> {
        Self::new(
            rollers.iter()
                .enumerate()
                .map(|(idx, roller)| NamedRoller::for_index(idx, roller))
                .collect()) }
    
    pub fn add(&mut self, roller: NamedRoller) -> &Self {
        self.inner.push(roller);
        self }
    
    pub fn add_with_name(&mut self, name: Name, roller: Rc<dyn Roller>) -> &Self {
        self.inner.push(NamedRoller::new(name, roller));
        self }
    
    pub fn add_numbered(&mut self, roller: Rc<dyn Roller>) -> &Self {
        self.inner.push(NamedRoller::for_index(self.inner.len(), roller));
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


struct NamedRoll {
    name: Name,
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