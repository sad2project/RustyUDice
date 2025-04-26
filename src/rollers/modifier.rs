use std::rc::Rc;
use crate::{
    Value, Values,
    random::Rng, 
    rollers::{SubRoll, SubRoller,DieRoll, Roll, Roller} };

pub struct ModifierRoller {
    roller: Rc<dyn SubRoller>,
    modifier: Value
}
impl ModifierRoller {
    pub fn new(roller: Rc<dyn SubRoller>, modifier: Value) -> Rc<Self> {
        Rc::new(Self{ roller, modifier }) }
}
impl Roller for ModifierRoller {    
    fn description(&self) -> String {
        let inner_desc = &self.roller.inner_description();
        format!("{} + {}", inner_desc, self.modifier)
    }

    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        ModifierRoll::new(self.roller.clone().inner_roll_with(rng), self.modifier.clone()) }
}
impl SubRoller for ModifierRoller {
    fn is_simple(&self) -> bool { true }

    fn inner_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> { 
        ModifierRoll::new(self.roller.clone().inner_roll_with(rng), self.modifier.clone()) }
}


pub struct ModifierRoll {
    inner_result: Box<dyn SubRoll>,
    modifier: Value
}
impl ModifierRoll {
    fn new(inner_result: Box<dyn SubRoll>, modifier: Value) -> Box<Self> {
        Box::new(Self { inner_result, modifier}) }
}
impl Roll for ModifierRoll {
    fn intermediate_results(&self) -> String {
        if self.inner_result.is_simple() {
            format!("{} + {}", self.inner_result.intermediate_results(), self.modifier.output()) }
        else {
            format!("({}) + {}", self.inner_result.intermediate_results(), self.modifier.output()) } }
    
    fn final_result(&self) -> String { self.totals().to_string() }
}
impl SubRoll for ModifierRoll {
    fn is_simple(&self) -> bool { true }
    
    fn rolled_faces(&self) -> Vec<&DieRoll> { (&self.inner_result).rolled_faces() }
    
    fn totals(&self) -> Values {
        let mut out = self.inner_result.totals();
        out.add_value(self.modifier.clone());
        out }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::rc::Rc;
    use crate::dice::{Die, Face};
    use crate::units::{BasicUnit, DNumUnit};
    use crate::{Unit, Value};
    use crate::rollers::SubRoller;

    fn make_test_die() -> (Rc<dyn Unit>, Rc<dyn Unit>, Rc<Die>) {
        let unit1 = BasicUnit::new("Successes".to_string(), "{} Successes".to_string(), false);
        let unit2 = DNumUnit::new();
        let die = Die::new("Mixer".to_string(), vec![
            Face::new("both".to_string(), vec![
                Value{ unit: unit1.clone(), value: 1},
                Value{ unit: unit2.clone(), value: 1}]),
            Face::new("basic".to_string(), vec![
                Value{ unit: unit2.clone(), value: 2}]),
            Face::new("successor".to_string(), vec![
                Value{ unit: unit1.clone(), value: 2}])
        ]);
        (unit1, unit2, die) }
    
    #[test]
    fn test_die() {
        let (unit1, unit2, die) = make_test_die();
        // This is not the real test; it merely exists to get rid of warnings
        assert!(unit1.deref() == unit2.deref());
        assert!(die.is_die())
    }
}