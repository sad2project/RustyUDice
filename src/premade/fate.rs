use std::rc::Rc;
use crate::{Unit, Value, dice::{Die, Face}, units::BasicUnit, clone_vec, Name};


pub fn build() -> (Rc<dyn Unit>, Rc<Die>) { 
    let unit = unit();
    let die = Die::new("Fate", faces(&unit));
    (unit, die) }


fn name(name: &str) -> Name {
    name.try_into().unwrap() } 


fn unit() -> Rc<dyn Unit> { BasicUnit::new(name("Shifts"), "{} Shifts", false) }


fn faces(unit: &Rc<dyn Unit>) -> Vec<Rc<Face>> {
    let face_plus = Face::with_one_val(name("+"), Value::new(unit, 1));
    let face_minus = Face::with_one_val(name("-"), Value::new(unit, -1));
    let face_blank = Face::blank(unit);
    clone_vec![
        face_plus,
        face_minus,
        face_blank ] }