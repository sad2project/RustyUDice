use std::{
    fmt::{Display, Error, Formatter},
    rc::Rc,
    ops::Deref };
use crate::{
    Values,
    dice::{Die, Face},
    random::Rng,
    rollers::{Roll, Roller} };
use crate::rollers::{ComposableRoll, ComposableRoller};

impl Roller for Die {
    fn description(&self) -> String {
        self.name.clone() }

    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        DieRoll::new(self.clone(), self.roll_face_with(rng) ) }
}
impl ComposableRoller for Die {
    fn is_simple(&self) -> bool { true }
    
    fn is_die(&self) -> bool { true }

    fn composable_roll(self: Rc<Self>, rng: Rng) -> Box<dyn ComposableRoll> {
        DieRoll::new(self.clone(), self.roll_face_with(rng))
    }
}


#[derive(Clone, Debug)]
pub struct DieRoll {
    die: Rc<Die>,
    face: Rc<Face>
}
impl DieRoll {
    pub fn new(die: Rc<Die>, face: Rc<Face>) -> Box<Self> {
        Box::new(Self{ die, face }) }
}
impl Roll for DieRoll {
    fn intermediate_results(&self) -> String { self.to_string() }

    fn final_result(&self) -> String {
        self.totals().to_string() }
}
impl ComposableRoll for DieRoll {
    fn is_simple(&self) -> bool { true }
    
    fn is_die(&self) -> bool { true }

    fn rolled_faces(&self) -> Vec<&DieRoll> {
        vec![self] }

    fn totals(&self) -> Values { self.face.deref().values.clone() }
}
impl Display for DieRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("{}:[{}]", self.die, self.face)) }
}

#[cfg(test)]
mod tests {
    use std::{
        rc::Rc };
    use crate::{
        Value, Values,
        dice::{Die, Face},
        random::Rng,
        relationships::DNumRelationship,
        rollers::{Roller, Roll} };
    use crate::random::default_rng;
    use crate::rollers::ComposableRoller;

    fn d2_test_die() -> Rc<Die> {
        let rel = DNumRelationship::new();
        let face1 = Face::new("1".to_string(), vec![Value{relationship: rel.clone(), value: 1}]);
        let face2 = Face::new("2".to_string(), vec![Value{relationship: rel.clone(), value: 2}]);
        Die::new("d2".to_string(), vec![face1, face2]) }

    fn always_2_rng() -> Rng { Rng::from_seed(2) }

    fn always_1_rng() -> Rng { Rng::from_seed(1) }

    #[test]
    fn d2_roll_totals() {
        let die: Rc<Die> = d2_test_die();
        let die_roller: Rc<dyn ComposableRoller> = die.clone();
        let one: &Values = &die.faces.get(0).unwrap().values;
        let two: &Values = &die.faces.get(1).unwrap().values;

        assert_eq!(die_roller.clone()
                       .composable_roll(always_1_rng()).totals(),
                   one.clone());
        assert_eq!(die_roller
                       .composable_roll(always_2_rng()).totals(),
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
        assert!(die.composable_roll(default_rng()).is_simple()) }

    #[test]
    fn d2_roll_rolled_faces() {
        let die_roll = d2_test_die().composable_roll(always_1_rng());
        let sut = die_roll.rolled_faces();
        assert_eq!(sut.len(), 1);
        assert_eq!(sut[0].intermediate_results(), "d2:[1]"); }

    #[test]
    fn d2_roll_final_result() {
        let die_roll = d2_test_die().roll_with(always_1_rng());
        assert_eq!(die_roll.final_result(), "1"); }
}