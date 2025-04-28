use std::rc::Rc;
use crate::{
    Value,
    dice::{Die, Face},
    units::BasicUnit};

type RUnit = Rc<dyn Unit>;
type RFace = Rc<Face>;
type RDie = Rc<Die>;


pub fn build() -> (Vec<RUnit>, Vec<RDie> { 
    let units = units();
    let (success, boon, delay, exert, comet, star, reroll) = units.clone();
    let (challenge_face, success_face, succ_boon_face, bane_face, boon_face, blank_face) = common_faces()
    let dice = vec![
        characteristic_die(success_face.clone(), boon_face.clone(), blank_face.clone()),
        challenge_die(challenge_face.clone(), bane_face.clone(), blank_face.clone(), success, boon, star),
        expertise_die(success_face.clone(), boon_face.clone(), blank_face.clone(), success, reroll, comet),
        fortune_die(success_face.clone(), boon_face.clone(), blank_face.clone()),
        misfortune_die(challenge_face.clone(), bane_face.clone(), blank_face.clone()),
        conservative_die(success_face.clone(), boon_face.clone(), succ_boon_face.clone(), blank_face.clone(), success, delay),
        reckless_die(succ_boon_face.clone(), bane_face.clone(), blank_face.clone(), success, boon, exert)
        ];
    (units.into().into(), dice) }


fn units() -> (Rc<dyn Unit>, Rc<dyn Unit>, Rc<dyn Unit>, Rc<dyn Unit>, Rc<dyn Unit>, Rc<dyn Unit>, Rc<dyn Unit>) {
    ( TieredUnit::pos_zero_neg("Successes".into(), "{} Successes".into(), "{} Successes".into(), "{|} Challenges".into()),
    TieredUnit::pos_neg("Boons".into(), "{} Boons".into(), "{|} Banes".into()),
    BasicUnit::new("Delay".into(), "{} Delay(s)".into(), true),
    BasicUnit::new("Exertion".into(), "{} Exertion".into(), true),
    BasicUnit::new("Sigmar's Comet".into(), "{} Sigmar's Comet(s)".into(), true),
    BasicUnit::new("Chaos Star".into(), "{} Chaos Star(s)".into(), true),
    BasicUnit::new("Reroll".into(), "Reroll {} Expertise Dice".into(), true) ) }


fn common_faces(success_unit: Rc<dyn Unit>, boon_unit: Rc<dyn Unit>) -> (Rc<Face>, Rc<Face>, Rc<Face>, Rc<Face>, Rc<Face>, Rc<Face>) {
    ( Face::new("Challenge", success_unit.clone(), -1),
    Face::new("Success", success_unit.clone(), 1),
    Face::new("Success + Boon", (success_unit, 1), (boon_unit.clone(), 1)),
    Face::new("Bane", boon_unit.clone(), -1),
    Face::with_one_val("Boon", boon_unit, 1),
    Face::blank()) }


fn characteristic_die(success: RFace, boon: RFace, blank: RFace) -> RDie {
    Die::new("Characteristic".into(), vec![success.clone(), success, boon, blank]) }


fn challenge_die(challenge: RFace, bane: RFace, blank: RFace, succ_unit: RUnit, boon_unit: RcUnit, star_unit: RUnit) -> RDie {
    let challengex2 = face_with_one_value("Challenge x2", succ_unit.clone().with_value(-2));
    Die::new("Challenge".into(), vec![
        challenge.clone(),
        challenge,
        challengex2.clone(),
        challengex2,
        bane,
        face_with_one_value("Bane x2", boon_unit.with_value(-2)),
        face_with_one_value("Chaos Star", star_unit.with_value(1)),
        blank]) }


fn expertise_die(success: RFace, boon: RFace, blank: RFace, succ_unit: RUnit, reroll_unit: RUnit, comet_unit: RUnit) -> RDie {
    let righteous = face_with_two_values("Righteous Success", succ_unit.with_value(1), reroll_unit.with_value(1));
    Die::new("Expertise".into(), vec![
        success,
        righteous,
        boon.clone(),
        boon,
        face_with_one_value("Sigmar's Comet", comet_unit, 1),
        blank]) }


fn fortune_die(success: RFace, boon: RFace, blank: RFace) -> RDie {
    Die::new("Fortune".into(), vec![
        success.clone(),
        success,
        boon,
        blank.clone(),
        blank.clone(),
        blank]) }


fn misfortune_die(challenge: RFace, bane: RFace, blank: RFace) -> RDie {
    Die::new("Misfortune".into(), vec![
        challenge.clone(),
        challenge,
        bane,
        blank.clone(),
        blank.clone(),
        blank]) }


fn conservative_die(success: RFace, boon: RFace, succ_boon: RFace, blank: RFace, succ_unit: RUnit, delay_unit: RUnit) -> RDie {
    let succ_delay = face_with_two_values("Success + Delay", succ_unit.with_value(1), delay_unit.with_value(1));
    Die::new("Conservative".into(), vec![
        success.clone(),
        success.clone(),
        success.clone(),
        success,
        boon.clone(),
        boon,
        succ_boon,
        succ_delay.clone(),
        succ_delay,
        blank]) }


fn reckless_die(succ_boon: RFace, bane: RFace, blank: RFace, succ_unit: RUnit, boon_unit: RUnit, exert_unit: RUnit) -> RDie {
    let successx2 = face_with_one_value("Success x2", succ_unit.with_value(2));
    let succ_exert = face_with_two_values("Success + Exertion", succ_unit.with_value(1), exert_unit.wiht_value(1));
    Die::new("Reckless".into(), vec![
        successx2.clone(),
        successx2,
        face_with_one_value("Boon x2", boon_unit.with_value(2)),
        succ_boon,
        bane.clone(),
        bane,
        succ_exert.clone(),
        succ_exert,
        blank.clone(),
        blank]) }