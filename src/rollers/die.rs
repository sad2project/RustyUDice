use std::{
    fmt::{Display, Error, Formatter},
    rc::Rc,
    ops::Deref };
use crate::{
    Values,
    dice::{Die, Face},
    random::Rng,
    rollers::{Roll, Roller} };
use crate::rollers::{SubRoll, SubRoller};


impl Roller for Die {
    fn description(&self) -> String {
        self.name.clone() }

    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        let face = self.roll_face_with(rng.clone());
        let initial_roll = DieRoll::new(self.clone(), face.clone());
        let mut output_roll: Box<dyn Roll> = initial_roll.clone();
        if initial_roll.should_explode() {
            output_roll = initial_roll.explode(rng); }
        output_roll }
}
impl SubRoller for Die {
    fn is_simple(&self) -> bool { true }
    
    fn is_die(&self) -> bool { true }

    fn inner_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        let face = self.roll_face_with(rng.clone());
        let initial_roll = DieRoll::new(self.clone(), face.clone());
        let mut output_roll: Box<dyn SubRoll> = initial_roll.clone();
        if initial_roll.should_explode() {
            output_roll = initial_roll.explode(rng); }
        output_roll }
}


#[derive(Clone, Debug)]
pub struct DieRoll {
    die: Rc<Die>,
    face: Rc<Face>
}
impl DieRoll {
    pub fn new(die: Rc<Die>, face: Rc<Face>) -> Box<Self> {
        Box::new(Self{ die, face }) }

    fn should_explode(&self) -> bool {
        face.value_for(&die.explode_on).is_some() }
    
    fn explode(self, rng: Rng) -> Box<ExplodedRoll> {
        let num_explosions = self.face.value_for(&self.die.explode_on).unwrap();
        let mut output_roll = ExplodedRoll::new(self.clone(), num_explosions);
        for _ in 0..num_explosions {
            output_roll.push(die.clone().inner_roll_with(rng.clone())); }
        output_roll }
}
impl Roll for DieRoll {
    fn intermediate_results(&self) -> String { self.to_string() }

    fn final_result(&self) -> String {
        self.totals().to_string() }
}
impl SubRoll for DieRoll {
    fn is_simple(&self) -> bool { true }

    fn rolled_faces(&self) -> Vec<&DieRoll> {
        vec![self] }

    fn totals(&self) -> Values { self.face.deref().values.clone() }
}
impl Display for DieRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("{}:[{}]", self.die, self.face)) }
}


#[derive(Clone)]
pub struct ExplodedRoll {
    pub original: Box<dyn SubRoll>,
    pub rolls: Vec<Box<dyn SubRoll>>
}
impl ExplodedRoll {
    fn new(trigger_roll: Box<dyn SubRoll>, num_explosions: usize) -> Box<Self> {
        Box::new(Self{ original: trigger_roll, rolls: Vec::with_capacity(num_explosions) }) }
    
    fn push(&mut self, roll: Rc<dyn SubRoll>) {
        self.rolls.push(roll); }
}
impl Roll for ExplodedRoll {
    fn intermediate_results(&self) -> String {
        if self.rolls.len() == 1 {
            format!(
                "{} => (exploded: {})", 
                self.original.inner_intermediate_results(), 
                self.rolls[0].inner_intermediate_results()) }
        else {
            let exploded_rolls = self.rolls.iter()
                .map(SubRoll::inner_intermediate_results)
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                "{} => (exploded {} times: {})",
                self.original, 
                self.rolls.len(),
                exploded_rolls) } }
    
    fn final_result(&self) -> String {
        self.totals.to_string() }
}
impl SubRoll for ExplodedRoll {
    fn is_simple(&self) -> bool { false }
    
    fn rolled_faces(&self) -> Vec<&DieRoll> {
        let mut faces = self.original.rolled_faces().into_iter();
        for roll in self.rolls.iter() {
            faces.chain(roll.rolled_faces()) }
        faces.collect() }
    
    fn totals(&self) -> Values {
        let mut values = self.original.totals();
        for roll in self.rolls.iter() {
            values.add_all_values(roll.totals()); }
        Values }
}


#[cfg(test)]
mod tests {
    use std::{
        rc::Rc };
    use crate::{
        Value, Values,
        dice::{Die, Face},
        random::Rng,
        units::DNumUnit,
        rollers::{Roller, Roll} };
    use crate::random::default_rng;
    use crate::rollers::SubRoller;

    fn d2_test_die() -> Rc<Die> {
        let rel = DNumUnit::new();
        let face1 = Face::new("1", vec![Value{ unit: rel.clone(), value: 1}]);
        let face2 = Face::new("2", vec![Value{ unit: rel.clone(), value: 2}]);
        Die::new("d2", vec![face1, face2]) }

    fn always_2_rng() -> Rng { Rng::from_seed(2) }

    fn always_1_rng() -> Rng { Rng::from_seed(1) }

    #[test]
    fn d2_roll_totals() {
        let die: Rc<Die> = d2_test_die();
        let die_roller: Rc<dyn SubRoller> = die.clone();
        let one: &Values = &die.faces.get(0).unwrap().values;
        let two: &Values = &die.faces.get(1).unwrap().values;

        assert_eq!(die_roller.clone()
                       .inner_roll_with(always_1_rng()).totals(),
                   one.clone());
        assert_eq!(die_roller
                       .inner_roll_with(always_2_rng()).totals(),
                   two.clone()); }

    #[test]
    fn d2_roll_intermediate_results() {
        let die = d2_test_die();
        assert_eq!(die.clone().roll_with(always_1_rng()).intermediate_results(), "d2:[1]");
        assert_eq!(die.roll_with(always_2_rng()).intermediate_results(), "d2:[2]"); }

    #[test]
    fn d2_description() {
        let die = d2_test_die();
        assert_eq!(die.description(), "d2"); }

    #[test]
    fn d2_is_simple() {
        let die = d2_test_die();
        assert!(die.is_simple()); }

    #[test]
    fn d2_roll_is_simple() {
        let die = d2_test_die();
        assert!(die.inner_roll_with(default_rng()).is_simple()) }

    #[test]
    fn d2_roll_rolled_faces() {
        let die_roll = d2_test_die().inner_roll_with(always_1_rng());
        let sut = die_roll.rolled_faces();
        assert_eq!(sut.len(), 1);
        assert_eq!(sut[0].intermediate_results(), "d2:[1]"); }

    #[test]
    fn d2_roll_final_result() {
        let die_roll = d2_test_die().roll_with(always_1_rng());
        assert_eq!(die_roll.final_result(), "1"); }
}
