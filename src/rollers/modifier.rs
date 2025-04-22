use std::rc::Rc;
use crate::{
    random::Rng, 
    rollers::{DieRoll, Roll, Roller, Value}, 
    str_util::wrapped_text, 
    Values };

pub struct ModifierRoller {
    roller: Rc<dyn ComposableRoller>,
    modifier: Value
}
impl ModifierRoller {
    pub fn new(roller: Rc<dyn ComposableRoller>, modifier: Value) -> Rc<Self> {
        Rc::new(Self{ roller, modifier }) }
}
impl Roller for ModifierRoller {    
    fn description(&self) -> String {
        let inner_desc = wrapped_text(&self.roller.description(), self.roller.is_simple());
        format!("{} + {}", inner_desc, self.modifier)
    }

    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        ModifierRoll::new(self.roller.clone().composable_roll(rng), self.modifier.clone()) }
}
impl ComposableRoller for ModifierRoller {
    fn is_simple(&self) -> bool { true }
    
    fn composable_roll(self: Rc<Self>, rng: Rng) -> Box<dyn ComposableRoll> { 
        ModifierRoll::new(self.roller.clone().composable_roll(rng), self.modifier.clone()) }
}


pub struct ModifierRoll {
    inner_result: Box<dyn ComposableRoll>,
    modifier: Value
}
impl ModifierRoll {
    fn new(inner_result: Box<dyn ComposableRoll>, modifier: Value) -> Box<Self> {
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
impl ComposableRoll for ModifierRoll {
    fn is_simple(&self) -> bool { true }
    
    fn rolled_faces(&self) -> Vec<&DieRoll> { (&self.inner_result).rolled_faces() }
    
    fn totals(&self) -> Values {
        let mut out = self.inner_result.totals();
        out.add_value(self.modifier.clone());
        out }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::dice::{Die, Face};
    use crate::relationships::{BasicRelationship, DNumRelationship};
    use crate::{Relationship, Value};

    fn test_die() -> (Rc<dyn Relationship>, Rc<dyn Relationship>, Rc<Die>) {
        let relationship1 = BasicRelationship::new("Successes".to_string(), "{} Successes".to_string(), false);
        let relationship2 = DNumRelationship::new();
        let die = Die::new("Mixer".to_string(), vec![
            Face::new("both".to_string(), vec![
                Value{relationship: relationship1.clone(), value: 1},
                Value{relationship: relationship2.clone(), value: 1}]),
            Face::new("basic".to_string(), vec![
                Value{relationship:relationship2.clone(), value: 2}]),
            Face::new("successor".to_string(), vec![
                Value{relationship: relationship1.clone(), value: 2}])
        ]);
        (relationship1, relationship2, die) }
}