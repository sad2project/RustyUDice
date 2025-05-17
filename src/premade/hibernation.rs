use std::rc::Rc;
use crate::{
    Unit, Value,
    dice::{Die, Face}, 
    units::BasicUnit, 
    storage::{DieStorage, SetId} };

// pub fn get(storage: impl DieStorage) -> (Rc<dyn Unit>, Rc<Die>) {
//     // check if storage contains it. If it does, rebuild it from that
//     let mut dice = storage.get_set_dice(SetId(1));
//     if dice.len() == 0 {
//         
//     }
//     // otherwise, build it fresh and store it.
//     build()
// } 


pub fn build() -> (Rc<dyn Unit>, Rc<Die>) { 
    let unit = unit();
    let die = Die::new("Hibernation", faces(&unit);
    (unit, die) }


fn unit() -> Rc<dyn Unit> {
    BasicUnit::new("Successes", "{} Successes", false) }


fn faces(unit: &Rc<dyn Unit>) -> Vec<&Rc<Face>> {
    let face_plus = &Face::with_one_val("+", Value::new(unit, 1));
    let face_minus = &Face::with_one_val("-", Value::new(unit, -1));
    vec![
        face_plus,
        face_plus,
        face_plus,
        face_minus,
        face_minus,
        &Face::blank(unit)] }