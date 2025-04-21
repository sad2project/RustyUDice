use std::rc::Rc;
use crate::{
    Value,
    dice::{Die, Face},
    relationships::BasicRelationship };

pub fn build() -> Vec<Rc<Die>> { vec![Die::new("Hibernation".to_string(), faces())] }

fn faces() -> Vec<Rc<Face>> {
    let relationship = BasicRelationship::new("Successes".to_string(), "{} Successes".to_string(), false);
    let face_plus = Face::new("+".to_string(), vec![Value{relationship: relationship.clone(), value: 1}]);
    let face_minus = Face::new("-".to_string(), vec![Value{relationship: relationship.clone(), value: -1}]);
    vec![
        face_plus.clone(),
        face_plus.clone(),
        face_plus,
        face_minus.clone(),
        face_minus,
        Face::new(" ".to_string(), vec![Value{relationship: relationship.clone(), value: 0}]) ] }