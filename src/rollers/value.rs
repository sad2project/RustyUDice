/// `ValueRoller` is a `Roller` that produces a constant value. It isn't really a "`Roller`" in the
/// sense that it "rolls" something to come up with a value. Instead, it is used for modifiers to
/// add or subtract from a roll.
///
/// The name is optional, but is recommended when `values` contains more than one `Value`. Without a
/// name, textual representations default to how `Values` would display it.
pub struct ValueRoller {  // TODO: rename to ModifierRoller?
    name: Option<Name>,
    value: Values  // TODO: rename to values
}
impl ValueRoller {
    pub fn named(name: Name, value: Values) -> Rc<Self> {
        rc(ValueRoller { name: Some(name), value }) }
    
    pub fn unnamed(value: Values) -> Rc<Self> {
        rc(ValueRoller { name: None, value }) }
}
impl Roller for ValueRoller {
    fn description(&self) -> String {
        self.name.map_or(self.value.to_string(), |name| name.deref().to_owned()) }

    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        ValueRoll::new(self.name.clone(), self.value.clone()) }
}
impl SubRoller for ValueRoller {
    fn is_simple(&self) -> bool { self.name.is_some() }

    fn inner_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        ValueRoll::new(self.name.clone(), self.value.clone()) }
}


struct ValueRoll {
    name: Option<Name>,
    value: Values
}
impl ValueRoll {
    pub fn new(name: Option<Name>, value: Values) -> Box<ValueRoll> {
        ValueRoll { name, value } }
}
impl Roll for ValueRoll {
    fn intermediate_results(&self) -> String {
        self.name.map_or(self.value.to_string(), |name| name.deref().to_owned()) }

    fn final_result(&self) -> String { self.intermediate_results() }
}
impl SubRoll for ValueRoll {
    fn is_simple(&self) -> bool { self.name.is_some() }

    fn rolled_faces(&self) -> Vec<&DieRoll> { Vec::with_capacity(0) }

    fn totals(&self) -> Values { self.value.clone() }
}