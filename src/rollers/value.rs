use std::{
    ops::Deref,
    rc::Rc };
use crate::{
    Name, Values,
    random::Rng,
    rollers::{DieRoll, Roller, SubRoller, Roll, SubRoll} };

/// `ValueRoller` is a `Roller` that produces a constant value. It isn't really a "`Roller`" in the
/// sense that it "rolls" something to come up with a value. Instead, it is used for modifiers to
/// add or subtract from a roll.
///
/// The name is optional, but is recommended when `values` contains more than one `Value`. Without a
/// name, textual representations default to how `Values` would display it.
pub struct ModifierRoller {
    name: Option<Name>,
    values: Values
}
impl ModifierRoller {
    /// Creates a named `ValueRoller` with the given `Name` and `Values`
    pub fn named(name: Name, value: Values) -> Rc<Self> {
        Rc::new(ModifierRoller { name: Some(name), values: value }) }
    
    /// Creates an unnamed `ValueRoller` with the given `Values`
    pub fn unnamed(value: Values) -> Rc<Self> {
        Rc::new(ModifierRoller { name: None, values: value }) }
}
impl Roller for ModifierRoller {
    /// Returns either the `String` version of the `name` (if `Some`), or the `to_string()` value of
    /// `values`
    fn description(&self) -> String {
        self.name.clone().map_or(self.values.to_string(), |name| name.deref().to_owned()) }

    fn roll_with(self: Rc<Self>, _rng: Rng) -> Box<dyn Roll> {
        ValueRoll::new(self.name.clone(), self.values.clone()) }
}
impl SubRoller for ModifierRoller {
    /// `true` if `name` is `Some(Name)`, else `false`
    fn is_simple(&self) -> bool { self.name.is_some() }

    fn inner_roll_with(self: Rc<Self>, _rng: Rng) -> Box<dyn SubRoll> {
        ValueRoll::new(self.name.clone(), self.values.clone()) }
}


struct ValueRoll {
    name: Option<Name>,
    value: Values
}
impl ValueRoll {
    pub fn new(name: Option<Name>, value: Values) -> Box<ValueRoll> {
        Box::new(ValueRoll { name, value }) }
}
impl Roll for ValueRoll {
    /// Returns either the `String` version of the `name` (if `Some`), or the `to_string()` value of
    /// `values`
    fn intermediate_results(&self) -> String {
        self.name.clone().map_or(self.value.to_string(), |name| name.deref().to_owned()) }

    /// Same as `intermediate_results()`
    fn final_result(&self) -> String { self.intermediate_results() }
}
impl SubRoll for ValueRoll {
    /// `true` if `name` is `Some(Name)`, else `false`
    fn is_simple(&self) -> bool { self.name.is_some() }

    /// Since there are no rolled faces, this returns an empty `Vec`
    fn rolled_faces(&self) -> Vec<&DieRoll> { Vec::with_capacity(0) }

    /// Returns `values`
    fn totals(&self) -> Values { self.value.clone() }
}