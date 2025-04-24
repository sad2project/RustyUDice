pub struct NamedRoller {
  name: Option<String>,
  roller: Rc<dyn Roller>
}
impl NamedRoller {
    pub fn new(name: String, roller: Rc<dyn Roller>) -> Self {
        Self { name: Some(name), roller } }
    
    pub fn numbered(roller: Rc<dyn Roller>) -> Self {
        Self { name: None, roller } }
    
    fn name(&self, index: usize) -> String {
        if self.name.is_none() { index.to_string() }
        else { self.name.clone() } } 
    
    fn description(&self, index: usize) -> String {
        format!("{}: {}", self.name(index), self.roller.description()) }
    
    fn roll_with(&self, index: usize, rng: Rng) -> NamedRoll {
        NamedRoll { name: self.name(index), roll: self.roller.clone().roll_with(rng) } }
}

pub struct NamedRoll {
    name: String,
    roll: Rc<dyn Roll>
}
impl NamedRoll {
    fn intermediate_results(&self) -> String {
        format!("{}: {}", self.name, self.roll.intermediate_results()) }
    
    fn final_result(&self) -> String {
        format!("{}: {}", self.name, self.roll.final_result()) }
}


// This can't be a proper `Roller`. It doesn't come up with a single result total. 
// It's used for rolling multiple things at once but not adding them up. 
pub struct MultiRoller {
    inner: Vec<NamedRoller>
}
impl MultiRoller {
    pub fn new(inner: Vec<NamedRoller>) -> Rc<Self> { Rc::new(MultiRoller{ inner }) }
    
    pub fn add(&mut self, roller: NamedRoller) -> &Self {
        self.inner.push(roller);
        self }
    
    pub fn add_numbered(&mut self, roller: Rc<dyn Roller>) -> &Self {
        self.inner.push(NamedRoller:numbered(roller);
        self }
}
impl Roller for MultiRoller {
    fn description(&self) -> String {
        self.inner.iter().enumerate()
          .map(|idx, roller| roller.description(idx))
          .collect()
          .join("\n") }
    
    fn roll_with(&self, rng: Rng) -> Rc<dyn Roll> {
        self.inner.iter().enumerate()
          .map(|idx, roller| roller.roll_with(idx, rng))
          .collect::<MultiRoll>()
          .rc() }
}


struct MultiRoll {
    inner: Vec<NamedRoll>
}
impl MultiRoll {
    fn rc(self) -> Rc<Self> { Rc::new(self) }
}
impl Roll for MultiRoll {
    fn intermediate_results(&self) -> String {
        self.inner.iter()
          .map(NamedRoll::intermediate_results)
          .collect()
          .join("\n") }
    
    fn final_result(&self) -> String {
        self.inner.iter()
          .map(NamedRoll::final_result)
          .collect()
          .join("\n") }
}
impl FromIterator<NamedRoll> for MultiRoll {
    fn from_iter<T: IntoIterator<Item=NamedRoll>>(iter: T) -> Self {
        MultiRoll { inner: iter.collect() } }
}