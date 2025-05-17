use std::rc::Rc;
use crate::{
    Unit, Value,
    dice::{Die, Face}, 
    units::{BasicUnit, TieredUnit} };

type RUnit = Rc<dyn Unit>;
type RFace = Rc<Face>;
type RDie = Rc<Die>;


pub fn build() -> (Vec<RUnit>, Vec<RDie>) { 
    let units = units();
    let (succ_unit, adv_unit, triumph_unit, force_unit) = units.clone();
    let (succ_face, succx2_face, fail_face, failx2_face, adv_face, advx2_face, threat_face, threatx2_face, succ_adv_face, fail_threat_face, blank_face) = common_faces(&succ_unit, &adv_unit);
    let dice: Vec<RDie> = vec![
        ability_die(&succ_face, &succx2_face, &adv_face, &advx2_face, &succ_adv_face, &blank_face),
        proficiency_die(&succ_face, &succx2_face, &adv_face, &advx2_face, &succ_adv_face, &blank_face),
        boost_die(&succ_face, &succx2_face, &adv_face, &advx2_face, &succ_adv_face, &blank_face),
        difficulty_die(&fail_face, &failx2_face, &threat_face, &threatx2_face, &fail_threat_face, &blank_face),
        challenge_die(&fail_face, &failx2_face, &threat_face, &threatx2_face, &fail_threat_face, &blank_face),
        setback_die(&fail_face, &failx2_face, &threat_face, &threatx2_face, &fail_threat_face, &blank_face),
        force_die(&force_unit) ];
    let arr_units: [RUnit; 7] = units.into();
    (Vec::from(&arr_units), dice) }


fn units() -> (RUnit, RUnit, RUnit, RUnit) {
    ( TieredUnit::pos_zero_neg("Success", "{} Successes", "{} Successes", "{|} Failures"),
    TieredUnit::pos_neg("Advantage", "{} Advantage", "{|} Threat"),
    TieredUnit::pos_neg("Triumph", "{} Triumph", "{|} Despair"),
    TieredUnit::pos_neg("Force", "{} Light Side", "{|} Dark Side") ) }


fn common_faces(success_unit: &RUnit, adv_unit: &RUnit) -> (RFace, RFace, RFace, RFace, RFace, RFace, RFace, RFace, RFace, RFace, RFace) {
    ( Face::with_one_val("Success", Value::new(success_unit, 1)),
    Face::with_one_val("Success x2", Value::new(success_unit, 2)),
    Face::with_one_val("Failure", Value::new(success_unit, -1)),
    Face::with_one_val("Failure x2", Value::new(success_unit, -2)),
    Face::with_one_val("Advantage", Value::new(adv_unit, 1)),
    Face::with_one_val("Advantage x2", Value::new(adv_unit, 2)),
    Face::with_one_val("Threat", Value::new(adv_unit, -1)),
    Face::with_one_val("Threat x2", Value::new(adv_unit, -2)),
    Face::with_two_vals("Success + Advantage", Value::new(success_unit, 1), Value::new(adv_unit, 1)),
    Face::with_two_vals("Failure + Threat", Value::new(success_unit, -1), Value::new(adv_unit, -1)),
    Face::blank(success_unit)) }


fn difficulty_die(fail_face: &RFace, failx2_face: &RFace, threat_face: &RFace, threatx2_face: &RFace, fail_threat_face: &RFace, blank_face: &RFace) -> RDie {
    Die::new("Difficulty", vec![
        fail_face,
        failx2_face,
        threat_face,
        threat_face,
        threat_face,
        threatx2_face,
        fail_threat_face,
        blank_face])
}