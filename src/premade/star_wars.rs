use std::rc::Rc;
use crate::{Unit, Value, dice::{Die, Face}, units::{BasicUnit, TieredUnit}, clone_vec, Name};

type RUnit = Rc<dyn Unit>;
type RFace = Rc<Face>;
type RDie = Rc<Die>;


pub fn build() -> (Vec<RUnit>, Vec<RDie>) { 
    let units = units();
    let (succ_unit, adv_unit, triumph_unit, force_unit) = units.clone();
    let (succ_face, succx2_face, fail_face, failx2_face, adv_face, advx2_face, threat_face, threatx2_face, succ_adv_face, fail_threat_face, blank_face) = common_faces(&succ_unit, &adv_unit);
    let dice: Vec<RDie> = vec![
        ability_die(&succ_face, &succx2_face, &adv_face, &advx2_face, &succ_adv_face, &blank_face),
        proficiency_die(&succ_face, &succx2_face, &adv_face, &advx2_face, &succ_adv_face, &blank_face, &triumph_unit),
        boost_die(&succ_face, &adv_face, &advx2_face, &succ_adv_face, &blank_face),
        difficulty_die(&fail_face, &failx2_face, &threat_face, &threatx2_face, &fail_threat_face, &blank_face),
        challenge_die(&fail_face, &failx2_face, &threat_face, &threatx2_face, &fail_threat_face, &blank_face, &triumph_unit),
        setback_die(&fail_face, &threat_face, &blank_face),
        force_die(&force_unit) ];
    let arr_units: [RUnit; 4] = units.into();
    (Vec::from(&arr_units), dice) }


fn name(name: &str) -> Name {
    name.try_into().unwrap() }


fn units() -> (RUnit, RUnit, RUnit, RUnit) {
    ( TieredUnit::pos_zero_neg(name("Success"), "{} Successes", "{} Successes", "{|} Failures"),
    TieredUnit::pos_neg(name("Advantage"), "{} Advantage", "{|} Threat"),
    TieredUnit::pos_neg(name("Triumph"), "{} Triumph", "{|} Despair"),
    TieredUnit::pos_neg(name("Force"), "{} Light Side", "{|} Dark Side") ) }


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


fn ability_die(succ_face: &RFace, succx2_face: &RFace, adv_face: &RFace, advx2_face: &RFace, succ_adv_face: &RFace, blank_face: &RFace) -> RDie {
    Die::new("Ability", clone_vec![
        succ_face,
        succ_face,
        succx2_face,
        adv_face,
        adv_face,
        succ_adv_face,
        advx2_face,
        blank_face ]) }


fn proficiency_die(succ_face: &RFace, succx2_face: &RFace, adv_face: &RFace, advx2_face: &RFace, succ_adv_face: &RFace, blank_face: &RFace, triumph_unit: &RUnit) -> RDie {
    Die::new("Proficiency", clone_vec![
        succ_face,
        succ_face,
        succx2_face,
        succx2_face,
        adv_face,
        adv_face,
        adv_face,
        succ_adv_face,
        succ_adv_face,
        succ_adv_face,
        Face::with_one_val("Triumph", Value::new(triumph_unit, 1)),
        blank_face ]) }


fn boost_die(succ_face: &RFace, adv_face: &RFace, advx2_face: &RFace, succ_adv_face: &RFace, blank_face: &RFace) -> RDie {
    Die::new("Boost", clone_vec![
        succ_face,
        succ_adv_face,
        adv_face,
        advx2_face,
        blank_face,
        blank_face ]) }


fn difficulty_die(fail_face: &RFace, failx2_face: &RFace, threat_face: &RFace, threatx2_face: &RFace, fail_threat_face: &RFace, blank_face: &RFace) -> RDie {
    Die::new("Difficulty", clone_vec![
        fail_face,
        failx2_face,
        threat_face,
        threat_face,
        threat_face,
        threatx2_face,
        fail_threat_face,
        blank_face]) }


fn challenge_die(fail_face: &RFace, failx2_face: &RFace, threat_face: &RFace, threatx2_face: &RFace, fail_threat_face: &RFace, blank_face: &RFace, triumph_unit: &RUnit) -> RDie {
    Die::new("Challenge", clone_vec![
        fail_face,
        failx2_face,
        threat_face,
        threat_face,
        threat_face,
        threatx2_face,
        fail_threat_face,
        blank_face ]) }


fn setback_die(fail_face: &RFace, threat_face: &RFace, blank_face: &RFace) -> RDie {
    Die::new("Setback", clone_vec![
        fail_face,
        fail_face,
        threat_face,
        threat_face,
        blank_face,
        blank_face
    ])
}


fn force_die(force_unit: &RUnit) -> RDie {
    let light = Face::with_one_val("Light", Value::new(force_unit, 1));
    let lightx2 = Face::with_one_val("Light x2", Value::new(force_unit, 2));
    let dark = Face::with_one_val("Dark", Value::new(force_unit, -1));
    let darkx2 = Face::with_one_val("Dark x2", Value::new(force_unit, -2));
    Die::new("Force", clone_vec![
        light,
        light,
        lightx2,
        lightx2,
        lightx2,
        dark,
        dark,
        dark,
        dark,
        dark,
        dark,
        darkx2 ]) }