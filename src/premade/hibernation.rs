use std::rc::Rc;
use crate::{
    Value,
    dice::{Die, Face},
    units::BasicUnit};

pub fn build() -> Vec<Rc<Die>> { vec![Die::new("Hibernation".to_string(), faces())] }

fn faces() -> Vec<Rc<Face>> {
    let unit = BasicUnit::new("Successes".to_string(), "{} Successes".to_string(), false);
    let face_plus = Face::new("+".to_string(), vec![Value{ unit: unit.clone(), value: 1}]);
    let face_minus = Face::new("-".to_string(), vec![Value{ unit: unit.clone(), value: -1}]);
    vec![
        face_plus.clone(),
        face_plus.clone(),
        face_plus,
        face_minus.clone(),
        face_minus,
        Face::new(" ".to_string(), vec![Value{ unit: unit.clone(), value: 0}]) ] }