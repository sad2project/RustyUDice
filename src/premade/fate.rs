use std::rc::Rc;
use crate::{Unit, dice::{Die, Face}, units::BasicUnit, Value};


pub fn build() -> (Rc<dyn Unit>, Rc<Die>) { 
    let unit = unit();
    let die = Die::new("Fate", faces(&unit).iter().collect());
    (unit, die) }


fn unit() -> Rc<dyn Unit> { BasicUnit::new("Shifts", "{} Shifts", false) }


fn faces(unit: &Rc<dyn Unit>) -> Vec<Rc<Face>> {
    let face_plus = Face::with_one_val("+", Value::new(unit, 1));
    let face_minus = Face::with_one_val("-", Value::new(unit, -1));
    let face_blank = Face::blank(unit);
    vec![
        face_plus.clone(),
        face_plus,
        face_minus.clone(),
        face_minus,
        face_blank.clone(),
        face_blank ] }