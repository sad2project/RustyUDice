use std::rc::Rc;
use crate::{Unit, Value, dice::{Die, Face}, units::{BasicUnit, TieredUnit}, clone_vec};

type RUnit = Rc<dyn Unit>;
type RFace = Rc<Face>;
type RDie = Rc<Die>;


pub fn build() -> (Vec<RUnit>, Vec<RDie>) { 
    let units = units();
    let (success, boon, delay, exert, comet, star, reroll) = units.clone();
    let (challenge_face, success_face, succ_boon_face, bane_face, boon_face, blank_face) = common_faces(&success, &boon);
    let dice: Vec<RDie> = vec![
        characteristic_die(&success_face, &boon_face, &blank_face),
        challenge_die(&challenge_face, &bane_face, &blank_face, &success, &boon, &star),
        expertise_die(&success_face, &boon_face, &blank_face, &success, &reroll, &comet),
        fortune_die(&success_face, &boon_face, &blank_face),
        misfortune_die(&challenge_face, &bane_face, &blank_face),
        conservative_die(&success_face, &boon_face, &succ_boon_face, &blank_face, &success, &delay),
        reckless_die(&succ_boon_face, &bane_face, &blank_face, &success, &boon, &exert) ];
    let arr_units: [RUnit; 7] = units.into();
    (Vec::from(&arr_units), dice) }


fn units() -> (RUnit, RUnit, RUnit, RUnit, RUnit, RUnit, RUnit) {
    ( TieredUnit::pos_zero_neg("Successes", "{} Successes", "{} Successes", "{|} Challenges"),
    TieredUnit::pos_neg("Boons", "{} Boons", "{|} Banes"),
    BasicUnit::new("Delay", "{} Delay(s)", true),
    BasicUnit::new("Exertion", "{} Exertion", true),
    BasicUnit::new("Sigmar's Comet", "{} Sigmar's Comet(s)", true),
    BasicUnit::new("Chaos Star", "{} Chaos Star(s)", true),
    BasicUnit::new("Reroll", "Reroll {} Expertise Dice", true) ) }


fn common_faces(success_unit: &RUnit, boon_unit: &RUnit) -> (RFace, RFace, RFace, RFace, RFace, RFace) {
    ( Face::with_one_val("Challenge", Value::new(success_unit, -1)),
    Face::with_one_val("Success", Value::new(success_unit, 1)),
    Face::with_two_vals("Success + Boon", Value::new(success_unit, 1), Value::new(boon_unit, 1)),
    Face::with_one_val("Bane", Value::new(boon_unit, -1)),
    Face::with_one_val("Boon", Value::new(boon_unit, 1)),
    Face::blank(success_unit)) }


fn characteristic_die(success: &RFace, boon: &RFace, blank: &RFace) -> RDie {
    Die::new("Characteristic", clone_vec![success, success, boon, blank]) }


fn challenge_die(challenge: &RFace, bane: &RFace, blank: &RFace, succ_unit: &RUnit, boon_unit: &RUnit, star_unit: &RUnit) -> RDie {
    let challenge_x2 = Face::with_one_val("Challenge x2", Value::new(&succ_unit, -2));
    Die::new("Challenge", clone_vec![
        challenge,
        challenge,
        challenge_x2,
        challenge_x2,
        bane,
        Face::with_one_val("Bane x2", Value::new(&boon_unit, -2)),
        Face::with_one_val("Chaos Star", Value::new(&star_unit, 1)),
        blank]) }


fn expertise_die(success: &RFace, boon: &RFace, blank: &RFace, succ_unit: &RUnit, reroll_unit: &RUnit, comet_unit: &RUnit) -> RDie {
    let righteous = Face::with_two_vals("Righteous Success", Value::new(succ_unit, 1), Value::new(reroll_unit, 1));
    Die::new("Expertise", clone_vec![
        success,
        righteous,
        boon,
        boon,
        Face::with_one_val("Sigmar's Comet", Value::new(&comet_unit, 1)),
        blank]) }


fn fortune_die(success: &RFace, boon: &RFace, blank: &RFace) -> RDie {
    Die::new("Fortune", clone_vec![
        success,
        success,
        boon,
        blank,
        blank,
        blank]) }


fn misfortune_die(challenge: &RFace, bane: &RFace, blank: &RFace) -> RDie {
    Die::new("Misfortune", clone_vec![
        challenge,
        challenge,
        bane,
        blank,
        blank,
        blank]) }


fn conservative_die(success: &RFace, boon: &RFace, succ_boon: &RFace, blank: &RFace, succ_unit: &RUnit, delay_unit: &RUnit) -> RDie {
    let succ_delay = Face::with_two_vals("Success + Delay", Value::new(&succ_unit, 1), Value::new(&delay_unit, 1));
    Die::new("Conservative", clone_vec![
        success,
        success,
        success,
        success,
        boon,
        boon,
        succ_boon,
        succ_delay,
        succ_delay,
        blank]) }

fn reckless_die(succ_boon: &RFace, bane: &RFace, blank: &RFace, succ_unit: &RUnit, boon_unit: &RUnit, exert_unit: &RUnit) -> RDie {
    let success_x2 = Face::with_one_val("Success x2", Value::new(&succ_unit, 2));
    let succ_exert = Face::with_two_vals("Success + Exertion", Value::new(&succ_unit, 1), Value::new(&exert_unit, 1));
    Die::new("Reckless", clone_vec![
        success_x2,
        success_x2,
        &Face::with_one_val("Boon x2", Value::new(&boon_unit, 2)),
        succ_boon,
        bane,
        bane,
        succ_exert,
        succ_exert,
        blank,
        blank]) }