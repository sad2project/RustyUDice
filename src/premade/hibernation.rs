use std::rc::Rc;
use crate::{
    Name, Unit, Value, clone_vec,
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
    let die = Die::new("Hibernation", faces(&unit));
    (unit, die) }


fn name(name: &str) -> Name {
    name.try_into().unwrap() }


fn unit() -> Rc<dyn Unit> {
    BasicUnit::new(name("Successes"), "{} Successes", false) }


fn faces(unit: &Rc<dyn Unit>) -> Vec<Rc<Face>> {
    let face_plus = Face::with_one_val(name("+"), Value::new(unit, 1));
    let face_minus = Face::with_one_val(name("-"), Value::new(unit, -1));
    clone_vec![
        face_plus,
        face_plus,
        face_plus,
        face_minus,
        face_minus,
        Face::blank(unit)] }