use std::{
    rc::Rc
};

use crate::{
    Values,
    random::Rng, 
    rollers::{Roller, Roll, SubRoller, SubRoll, DieRoll} };


/// When it comes to having a long stream of numbers being added and subtracted together, especially
/// when you want to represent that equation again later in text, you need to know whether you're
/// displaying the first value, an addition, or a subtraction. If you don't distinguish between them,
/// you end up with clunky representations that differ from the original intent. This type
/// encapsulates the three main representations in order to display correctly.
enum RollerMathType {
    First(Rc<dyn SubRoller>),
    Add(Rc<dyn SubRoller>),
    Subtract(Rc<dyn SubRoller>)
}
impl RollerMathType {
    fn description(&self) -> String {
        match self {
            RollerMathType::First(roller) => roller.inner_description(),
            RollerMathType::Add(roller) => format!(" + {}", roller.inner_description()),
            RollerMathType::Subtract(roller) => format!(" - {}", roller.inner_description()) } }
    
    fn roll_with(&self, rng: Rng) -> RollMathType {
        match self {
            RollerMathType::First(roller) => RollMathType::First(roller.clone().inner_roll_with(rng)),
            RollerMathType::Add(roller) => RollMathType::Add(roller.clone().inner_roll_with(rng)),
            RollerMathType::Subtract(roller) => RollMathType::Subtract(roller.clone().inner_roll_with(rng)) } }
}


/// A `Roller` that encapsulates a series of additions and subtractions of other `Roller`s.
pub struct MathRoller {
    inner: Vec<RollerMathType>
}
impl MathRoller {
    /// Creates a new `MathRoller`, adding the results of the 2 given rollers together
    pub fn add(lhs: Rc<dyn SubRoller>, rhs: Rc<dyn SubRoller>) -> Rc<Self> {
        Rc::new(Self{ inner: vec![RollerMathType::First(lhs), RollerMathType::Add(rhs)] }) }
    
    /// Creates a new `MathRoller`, subtracting the results of the 2nd given roller from the first
    pub fn subtract(lhs: Rc<dyn SubRoller>, rhs: Rc<dyn SubRoller>) -> Rc<Self> {
        Rc::new(Self{ inner: vec![RollerMathType::First(lhs), RollerMathType::Subtract(rhs)] }) }
    
    /// Adds the given roller's results to the results of the rest of this roller
    pub fn plus(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(RollerMathType::Add(roller));
        self }
    
    /// Adds the given modifier to this roller's results
    pub fn plus_modifier(mut self, modifier: Values) -> Self {
        self.inner.push(RollerMathType::Add(modifier.as_roller()));
        self }
    
    /// Adds the given modifier to this roller's results, using the given name as a display value
    /// for the description and intermediate results
    pub fn plus_named_modifier(mut self, modifier_name: Name, modifier: Values) -> Self {
        self.inner.push(RollerMathType::Add(modifier.as_roller_with_name(modifier_name)));
        self }
    
    /// Adds all of the given rollers to the results of the rest of this roller
    pub fn plus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(RollerMathType::Add));
        self }
    
    /// Subtracts the given roller's result to the results of the rest of this roller
    pub fn minus(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(RollerMathType::Subtract(roller));
        self }
    
    /// Subtracts the given modifier from this roller's results
    pub fn minus_modifier(mut self, modifier: Values) -> Self {
        self.inner.push(RollerMathType::Add(modifier.as_roller()));
        self }
    
    /// Subtracts the given modifier from this roller's results, using the given name as a display
    /// value for the description and intermediate results
    pub fn minus_named_modifier(mut self, modifier_name: Name, modifier: Values) -> Self {
        self.inner.push(RollerMathType::Add(modifier.as_roller_with_name(modifier_name)));
        self }
    
    /// Subtracts all of the given rollers from the results of the rest of this roller
    pub fn minus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(RollerMathType::Subtract));
        self }
}
impl Roller for MathRoller {
    fn description(&self) -> String {
        self.inner.iter()
            .map(RollerMathType::description)
            .collect::<Vec<String>>()
            .join("") }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        MathRoll::new(self.inner.iter().map(|roller| roller.roll_with(rng.clone()))) }
}
impl SubRoller for MathRoller {
    fn is_simple(&self) -> bool { false }

    fn inner_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        MathRoll::new(self.inner.iter().map(|roller| roller.roll_with(rng.clone()))) }
}


/// See `RollerMathType`'s documentation 
enum RollMathType {
    First(Box<dyn SubRoll>),
    Add(Box<dyn SubRoll>),
    Subtract(Box<dyn SubRoll>)
}
impl RollMathType {
    fn intermediate_results(&self) -> String {
        match self {
            RollMathType::First(roll) => roll.inner_intermediate_results(),
            RollMathType::Add(roll) => format!(" + {}", roll.inner_intermediate_results()),
            RollMathType::Subtract(roll) => format!(" - {}", roll.inner_intermediate_results()) } }
    
    fn rolled_faces(&self) -> Vec<&DieRoll> {
        match self {
            RollMathType::First(roll) 
            | RollMathType::Add(roll) 
            | RollMathType::Subtract(roll) => roll.rolled_faces() } }
    
    fn totals(&self) -> Values {
        match self {
            RollMathType:: First(roll) 
            | RollMathType::Add(roll) => roll.totals(),
            RollMathType::Subtract(roll) => -roll.totals() } }
}


struct MathRoll {
    inner: Vec<RollMathType>
}
impl MathRoll {
    pub fn new(inner: impl IntoIterator<Item=RollMathType>) -> Box<MathRoll> {
        Box::new(MathRoll { inner: inner.into_iter().collect() }) }
}
impl Roll for MathRoll {
    fn intermediate_results(&self) -> String {
        self.inner.iter()
            .map(RollMathType::intermediate_results)
            .collect() }

    fn final_result(&self) -> String {
        self.totals().to_string() }
}
impl SubRoll for MathRoll {
    fn is_simple(&self) -> bool { false }

    fn rolled_faces(&self) -> Vec<&DieRoll> {
        let mut output = Vec::with_capacity(self.inner.len());
        for inner_roll in self.inner.iter() {
            output.extend(inner_roll.rolled_faces()); }
        output }

    fn totals(&self) -> Values {
        self.inner.iter()
            .map(RollMathType::totals)
            .collect::<Values>() }
}