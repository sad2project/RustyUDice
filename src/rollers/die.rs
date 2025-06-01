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


/// The result of rolling a `Die` (unless the `Die` ends up exploding)
#[derive(Clone, Debug)]
pub struct DieRoll {
    die: Rc<Die>,
    face: Rc<Face>
}
impl DieRoll {
    pub(crate) fn new(die: Rc<Die>, face: Rc<Face>) -> Box<Self> {
        Box::new(Self{ die, face }) }

    fn should_explode(&self) -> bool {
        if let Some(explode_on) = &self.die.explode_on {
            self.face.value_for(explode_on).is_some() }
        else {
            false } }

    fn explode(self: Box<DieRoll>, rng: Rng) -> Box<ExplodedRoll> {
        let explode_on = self.die.explode_on.as_ref().unwrap();
        let num_explosions = self.face.value_for(explode_on).unwrap();
        let die = self.die.clone();
        let mut output_roll = ExplodedRoll::new(self.clone(), num_explosions as usize);
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


/// The result of a `Die`'s roll if it explodes (gets a result that gets a "roll again and add that
/// result on top"). Contains the triggering roll as well as all the additional rolls that may have
/// been triggered directly (if any of those rolls trigger additional explosions, they become their
/// own `ExplodedRoll`).
pub struct ExplodedRoll {
    triggering_roll: Box<dyn SubRoll>,
    triggered_rolls: Vec<Box<dyn SubRoll>>
}
impl ExplodedRoll {
    fn new(trigger_roll: Box<dyn SubRoll>, num_explosions: usize) -> Box<Self> {
        Box::new(Self{ triggering_roll: trigger_roll, triggered_rolls: Vec::with_capacity(num_explosions) }) }
    
    fn push(&mut self, roll: Box<dyn SubRoll>) {
        self.triggered_rolls.push(roll); }
}
impl Roll for ExplodedRoll {
    fn intermediate_results(&self) -> String {
        if self.triggered_rolls.len() == 1 {
            format!(
                "{} => (exploded: {})",
                self.triggering_roll.inner_intermediate_results(),
                self.triggered_rolls[0].inner_intermediate_results()) }
        else {
            let exploded_rolls = self.triggered_rolls.iter()
                .map(|roll| roll.inner_intermediate_results())
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                "{} => (exploded {} times: {})",
                self.triggering_roll.intermediate_results(),
                self.triggered_rolls.len(),
                exploded_rolls) } }
    
    fn final_result(&self) -> String {
        self.totals().to_string() }
}
impl SubRoll for ExplodedRoll {
    fn is_simple(&self) -> bool { false }
    
    fn rolled_faces(&self) -> Vec<&DieRoll> {
        // Gotta box it so that it can be dynamic, which allows the chaining later to be of the same type
        let mut faces: Box<dyn Iterator<Item=&DieRoll>> = Box::new(self.triggering_roll.rolled_faces().into_iter()); 
        for roll in self.triggered_rolls.iter() {
            // build up one big iterator via chaining
            faces = Box::new(faces.chain(roll.rolled_faces())) }
        faces.collect()
    }
    
    fn totals(&self) -> Values {
        let mut values = self.triggering_roll.totals();
        for roll in self.triggered_rolls.iter() {
            values.add_all_values(roll.totals()); }
        values }
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
