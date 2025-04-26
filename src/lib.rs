#![feature(unsigned_is_multiple_of)]

/// The UDice system is a dice-building and rolling library. The U stands for
/// Universal, since it's meant to support any kind of dice system that's ever
/// been invented (in theory). It can even represent a random table right in a
/// single die (though some would likely be better with multiple dice, if the
/// original table was built around multiple dice).
///
/// First, you have the `Die`, which has a name and some `Face`s. Each face has
/// a name and some `Value`s. None of this should be surprising, other than maybe
/// a `Face` having multiple `Value`s. There are plenty of dice systems out there
/// that have multiple symbols with different meanings. 
///
/// How do these `Value`s work, though? They have 2 fields: a `Relationship` and
/// an amount. The `Relationship` is pretty much the same idea as the 
/// aforementioned symbol, except that in some systems, different symbols can
/// work on the same `Relationship` (such as the concept of bane and boon symbols;
/// They're different symbols, but they're positive and negative versions of the
/// same `Relationship`. If you roll one of each, they cancel out). 
///
/// That's the primary part of the system, but the next part is how you get all
/// to actually give you random results. That's where `Roller`s and `Roll`s come
/// in. A `Roller` is what collects dice and modifiers together to form the hand
/// of dice. Then you tell it to `roll()`, and it produces a `Roll`, which is the
/// collection of all the `Face`s and `Value`s "rolled". It can describe all the
/// individual rolls of all the individual dice, along with their modifiers, plus 
/// it can give the final totals and results of the roll.
///
/// At this point in time, there are some decisions made to simplify the system, 
/// largely because I don't know of any dice systems out there that don't work
/// with the simple version, but I have thought of ways to make the system more
/// universal if needed. The simplifications are that `Relationship`s always use
/// an integer number for the amount, and that the only math used on those numbers
/// is adding and subtracting (using the addition of negative numbers).
/// This may change in the future, but I doubt it. The addition of other mathematical
/// operations is  far more likely than needing something other than integers, but
/// we'll see.

use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    rc::Rc };
use std::fmt::Debug;
use std::ops::Neg;

pub mod dice;
pub mod premade;
pub mod units;
pub mod rollers;
pub mod random;
mod u64_gen;


pub trait Unit: Debug + Display {
    fn id(&self) -> u64;
    /// If the relationship's outcome should be ignored (such as everything being 
    /// cancelled out, i.e. banes and boons totalling to zero), then return an
    /// empty String, and the display system should ignore it.
    /// TODO: decide if we'd rather use Option<String>
    fn output_for(&self, total: i32) -> String;
}

impl PartialEq for &dyn Unit {
    fn eq(&self, other: &Self) -> bool { self.id() == other.id() }
}


/// `Value` is essentially a key-value entry tying a number to a `Relationship`
/// It's first main use is for `Face` to hold what values it's worth. Its second
/// is for `Roll`s and totalling up their values.
#[derive(Clone, Debug)]
pub struct Value {
    pub unit: Rc<dyn Unit>,
    pub value: i32,
}
impl Value {
   fn add(&mut self, other: Value) -> bool {
      if self.has_same_unit(&other) {
         self.value += other.value;
         true }
      else { 
          false } }
   
   fn subtract(&mut self, other: Value) -> bool {
       if self.has_same_unit(&other) {
           self.value -= other.value;
           true }
       else {
           false } } 
    
   pub fn has_same_unit(&self, other: &Value) -> bool {
      self.unit.deref() == other.unit.deref() }

   pub fn output(&self) -> String { self.unit.output_for(self.value) }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.unit, self.value))
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.has_same_unit(other) && self.value == other.value }
}
impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Value { unit: self.unit, value: self.value * -1 } }
}


/// `Values` serves two purposes that are intrinsically linked, though they might be
/// separated later.
/// 
/// First, it serves to hold all the different `Value`s for a `Face`.
/// In that purpose, it really doesn't need any special functionality other than to
/// give a way to iterate over those `Value`s.
///
/// Second, it serves `Roller`s for totalling up faces rolled. For that, it has the
/// add_value() and add_all_values() methods, which add other `Value`s to the current one.
/// If the `Relationship` on the `Value` being added isn't already in this `Values`, then
/// it's inserted. Otherwise, it adds the value of the `Value` to the already existing
/// amount. You have to make sure you get a clone of the very first one so that you're not
/// mutating one that actually belongs to a Face. Luckily, `Die`'s implementation of
/// `Roller` and it's `Roll` type do this for you, so you don't have to worry about it.
/// 
/// `Values` uses just a `Vec` of `Value`s as a key-value store of Relationship to their 
/// totals because the absolute highest number of `Relationship`s I ever expect to be 
/// associated with a roll is 5, if that. Linear searches of a list of 5 or fewer values
/// may even be faster than using any typical kind of map. Plus, the fact that I can't
/// use `Relationship` as a key AND use it with `dyn` meant `HashMap` was out.
#[derive(Clone, Debug, PartialEq)]
pub struct Values { values: Vec<Value> }
impl Values {
    pub fn new() -> Self { 
        Self { values: Vec::with_capacity(1) } }
    
    pub fn with_capacity(capacity: usize) -> Self { 
        Self { values: Vec::with_capacity(capacity) } }
    
    pub fn add_value(&mut self, value: Value) {
        for val in self.values.iter_mut() { 
            // Value::add() returns whether it worked, which requires the same Relationship 
            // Therefore, if it did, we're done
            if val.add(value.clone()) { return; } }
        // If we haven't returned yet, that means that there aren't any Values with the same
        // Relationship in the collection yet, so we need to actually insert it
        self.values.push(value); }
    
    pub fn add_all_values(&mut self, values: Values) {
        for value in values.into_iter() {
            self.add_value(value.clone()); } }
    
    pub fn subtract_value(&mut self, value: Value) {
        for val in self.values.iter_mut() {
            if val.subtract(value.clone()) { return; } }
        self.values.push(value); }
    
    pub fn subtract_all_values(&mut self, values: Values) {
        for value in values.into_iter() {
            self.subtract_value(value.clone()) } }
    
    pub fn get(&self, unit: Rc<dyn Unit>) -> Option<i32> {
        for value in &self.values {
            if value.unit.deref() == unit.deref() { 
                return Some(value.value) } }
        None } 
}
impl Neg for Values {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Values { values: self.values.into_iter().map(Value::neg).collect() } }
}
impl IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;
    fn into_iter(self) -> Self::IntoIter { self.values.into_iter() }
}
impl <'a> IntoIterator for &'a Values {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;
    fn into_iter(self) -> <&'a Values as IntoIterator>::IntoIter { self.values.iter() }
}
impl <'a> IntoIterator for &'a mut Values {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;
    fn into_iter(self) -> <&'a mut Values as IntoIterator>::IntoIter { self.values.iter_mut() }
}
impl FromIterator<Values> for Values {
    fn from_iter<T: IntoIterator<Item=Values>>(iter: T) -> Self {
        iter.into_iter().fold(
            Values::new(), 
            |mut a, b| {a.add_all_values(b); a}) }
}
impl From<Value> for Values {
    fn from(value: Value) -> Self {
        Self { values: vec![value] } }
}
impl From<&Value> for Values {
    fn from(value: &Value) -> Self {
        Self { values: vec![value.clone()] } }
}
impl From<Vec<Value>> for Values {
    fn from(values: Vec<Value>) -> Self {
        Self { values } }
}
impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = self.values.iter()
            .map(Value::output)
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(&text)
    }
}

// TODO: Make a bunch of premade dice:
//     - d2, d3, d4, d6, d8, d10, d12, d20, d30, d50, d100
//     - dice for my board game
//     - dice for Fate
//     - dice for Warhammer?
//     - dice for Star Wars?
// Make them at compile time? To do so, we'll need to add a lot of `const` keywords
// feature-gate different games? Otherwise, simply provide a module (crate?) for each
// group with factories
// When generating all the basic dice, create the d100 faces list first, then use 
// subslices for each of the smaller dice, since faces are Rc'd anyway.