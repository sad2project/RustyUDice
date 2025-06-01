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
    fmt::{Display, Debug, Formatter},
    ops::{Deref, Neg},
    rc::Rc };
use crate::rollers::{SubRoller, ValueRoller};

pub mod dice;
pub mod premade;
pub mod units;
pub mod rollers;
pub mod random;
pub mod storage;

#[macro_export]
macro_rules! clone_vec {
    ($item:expr) => { 
        vec![$item.clone()] };
    ($($items:expr),+) => {
        vec![$($items.clone()),+] }; }


const MAX_NAME_LEN: usize = 35;


/// Error possibilities for illegal names
#[derive(Debug)]
pub enum NameError {
    /// If the name is literally empty or is all whitespace, that's an Empty error
    Empty,
    /// If the name is longer than 35 characters, it's too long
    TooLong
}


/// `Name` is a `String` wrapper for making valid names of things. It makes sure the string isn't
/// empty and isn't too short. 
#[derive(Clone, Debug)]
pub struct Name {
    val: String
}
impl Name {
    /// Take something that can make a String, validate it and either create a new Name or return a
    /// NameError.
    pub fn new(val: impl Display) -> Result<Name, NameError> {
        Ok(Name { val: Name::validate( val.to_string() )? }) }
    
    /// Takes a number and turns it into a String
    pub fn from_num(num: usize) -> Self {
        Name { val: num.to_string() } }
    
    fn validate(val: String) -> Result<String, NameError> {
        if val.trim().is_empty() {
            Err(NameError::Empty) }
        else if val.len() > MAX_NAME_LEN {
            Err(NameError::TooLong) }
        else { Ok(val) } }
}
impl TryFrom<String> for Name {
    type Error = NameError;
    fn try_from(value: String) -> Result<Name, NameError> {
        Name::new(value) }
}
impl TryFrom<&str> for Name {
    type Error = NameError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Name::new(value.to_string()) }
}
impl From<usize> for Name {
    fn from(value: usize) -> Name {
        Name::from_num(value) }
}
impl Deref for Name {
    type Target = str;
    fn deref(&self) -> &str {
        &self.val }
}
impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.val) }
}


pub trait Unit: Debug + Display {
    fn id(&self) -> u64;
    /// If the relationship's outcome should be ignored (such as everything being 
    /// cancelled out, i.e. banes and boons totalling to zero), then return an
    /// empty String, and the display system should ignore it.
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
    fn new(unit: &Rc<dyn Unit>, value: i32) -> Self {
        Self{ unit: unit.clone(), value } }
    
    fn add(&mut self, other: i32) {
        self.value += other; }
    
    // TODO: determine if this is used
    fn subtract(&mut self, other: i32) {
        self.value -= other; } 
    
    /// Checks if the `Unit` that makes up this `Value` is the same as the other `Value`'s
    pub fn has_same_unit(&self, other: &Value) -> bool {
        self.unit.deref() == other.unit.deref() }
    
    /// Checks if the `Unit` that makes up this `Value` is the same as the given one
    pub fn is_for_unit(&self, unit: &Rc<dyn Unit>) -> bool {
        self.unit.deref() == unit.deref() }
    
    /// Generates the output from the `Unit` within using the `value` within as the total
    pub fn output(&self) -> String { self.unit.output_for(self.value) }
    
    /// Creates an unnamed `ValueRoller` from this
    pub fn to_roller(self) -> Rc<dyn SubRoller> {
        Values::from(self).to_roller() }
    
    /// Creates a named `ValueRoller` from this
    pub fn to_roller_with_name(self, name: Name) -> Rc<dyn SubRoller> {
        Values::from(self).to_roller_with_name(name) }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.unit, self.value)) }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.has_same_unit(other) && self.value == other.value }
}
impl Neg for Value {
    type Output = Self;

    /// Inverts the value within
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
    /// Creates an empty `Values` (with a capacity of 1), which expects that you will start adding 
    /// `Value`s to it
    pub fn new() -> Self { 
        Self { values: Vec::with_capacity(1) } }
    
    /// Creates an empty `Values` with the given capacity, which expects that you will start adding
    /// `Value`s to it
    pub fn with_capacity(capacity: usize) -> Self { 
        Self { values: Vec::with_capacity(capacity) } }
    
    /// Add (insert) a `Value` into this. If there is already a `Value` with the same `Unit`, then
    /// the value of the given `Value` is added to the existing one.
    pub fn add_value(&mut self, value: Value) {
        for val in self.values.iter_mut() { 
            if val.has_same_unit(&value) { 
                val.add(value.value);
                return; } }
        // If we haven't returned yet, that means that there aren't any Values with the same
        // Relationship in the collection yet, so we need to actually insert it
        self.values.push(value); }
    
    /// Add (insert) the given `Value`s into this. If there is already a `Value` that has a matching
    /// `Unit` as one of the given `Value`s, the value of the given `Value` is add to the existing
    /// one.
    pub fn add_all_values(&mut self, values: Values) {
        for value in values.into_iter() {
            self.add_value(value.clone()); } }
    
    /// Subtract the value of the given `Value`. If there's an existing `Value` with the same `Unit`,
    /// it subtracts the value of the given `Value` from the existing one. If there isn't, a 
    /// negative version of the given `Value` is inserted.
    pub fn subtract_value(&mut self, value: Value) {
        self.add_value(-value); }
    
    /// Subtract the values of the all the `Value`s. If there are existing`Value`s with the same 
    /// `Unit`, it subtracts the value of the given `Value`s from the existing ones. For those that
    /// don't have an existing `Value` that matches the `Unit`, it inserts a negative version of the
    /// `Value`.
    pub fn subtract_all_values(&mut self, values: Values) {
        self.add_all_values(-values); }
    
    /// Returns the `Value` that has the same `Unit` as the given one, if any.
    pub fn value_for(&self, unit: &Rc<dyn Unit>) -> Option<i32> {
        for value in &self.values {
            if value.is_for_unit(&unit) {
                return Some(value.value) } }
        None }
    
    /// Creates an unnamed ValueRoller from the `Values`
    pub fn to_roller(self) -> Rc<dyn SubRoller> {
        ValueRoller::unnamed(self) }
    
    /// Creates a named `ValueRoller` from the `Values`
    pub fn to_roller_with_name(self, name: Name) -> Rc<dyn SubRoller> {
        ValueRoller::named(name, self) }
}
impl Neg for Values {
    type Output = Self;
    /// Returns a new `Values` where all the values of all the `Value`s are negated
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
        f.write_str(&text) }
}
