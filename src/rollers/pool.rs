use std::{
    cmp::Ordering,
    rc::Rc,
};
use crate::{
    {Unit, Values},
    rollers::{DieRoll, Roll, Roller, SubRoll, SubRoller},
    random::Rng,
    str_util::wrapped_text
};
use self::Strategy::*;

/// The `PoolRoller` uses a `Strategy`  in order to implement the  "drop lowest"
/// and "drop highest" options.  There's also the default "keep all" option. For
/// the options that drop dice, it needs a number of how many to drop, along with 
/// a list of `Relationship`s by which to sort the rolls in order to determine 
/// which are the lowest or highest. 
pub enum Strategy {
    DropLowest{ count: u8, order_by: Vec<Rc<dyn Unit>> },
    DropHighest{ count: u8, order_by: Vec<Rc<dyn Unit>> },
    KeepAll
}
impl Strategy {
    pub fn count(&self) -> u8 {
        match self {
            DropLowest{count, order_by: _} => { *count },
            DropHighest{count, order_by: _} => { *count },
            KeepAll => { 0 } } }
    
    pub fn is_simple(&self) -> bool {
        match self { 
            KeepAll => true,
            _ => false } }
    
    /// Uses the order_by field in this instance to act as a comparator for sorting Rolls.
    /// If a roll doesn't have a Value for one of the Relationships, it uses a default value of 0.
    /// This seems likes the most often correct behavior for sorting Rolls. The other obvious 
    /// behavior that might come up is to default to the MIN value. This may be changed if it
    /// turns out that this is the only actual expected behavior. Otherwise, if it turns out that
    /// we need to support both options, we'll need a field in Relationship to hold the default 
    /// value, either as an i32 (most flexible) or enum with the options of Zero and Min (more 
    /// compact, maybe).
    fn order_comparator(&self, roll1: &Box<dyn SubRoll>, roll2: &Box<dyn SubRoll>) -> Ordering {
        match self {
            KeepAll => { Ordering::Equal }
            DropLowest {count: _, order_by: order}
            | DropHighest {count: _, order_by: order} => {
                let roll1_vals = roll1.totals();
                let roll2_vals = roll2.totals();
                for relationship in order {
                    // Treat an absence of a Value for a Relationship as a 0 for it. 
                    // See function doc for more details
                    let r1val = roll1_vals.get(relationship.clone()).unwrap_or(0);
                    let r2val = roll2_vals.get(relationship.clone()).unwrap_or(0);
                    if r1val == r2val { continue; }
                    else { return Ord::cmp(&r1val, &r2val) }
                }
                Ordering::Equal } } }

    /// needed for the `PoolRoll` to describe itself properly
    fn descriptor(&self) -> String {
        match self {
            KeepAll => String::new(), 
            DropLowest{ count, order_by: _} => 
                if *count == 1 { String::from(" drop lowest") }
                else { format!(" drop lowest {}", count) },
            DropHighest{ count, order_by: _} => 
                if *count == 1 { String::from(" drop highest") }
                else { format!(" drop highest {}", count) } } } 
}


/// `PoolRoller` is a `Roller` that is used to do any rolling that involves 
/// multiple of the same roll or dice that will be added up, along with the 
/// option to drop some of those rolls based on which rolls are the lowest 
/// or highest. 
///
/// The classic example is old-school D&D's stat roll of 4d6 drop lowest,
/// which gives you rolls between 3 and 18 with a normal distribution, but
/// skewed a little higher.
///
/// Again, you can drop the lowest OR the highest, and it can be as many as 
/// you wish, up to one less than the number rolls you're rolling. Note that
/// it says "rolls" that you're rolling, not "dice". That's because you can 
/// do any kind of roll multiple times. For example, if you have an attack 
/// roll you're taking with advantage, you can set it up to do a d20 plus the
/// attack modifier twice and choose the higher of the two. Or, if a game is
/// a dice-pool kind of game where your attack roll uses 6d6, and you have
/// an equivalent to D&D's advantage where you roll twice and take the higher
/// result, you can build this with two `PoolRoller`s. One for the 6d6
/// (without dropping anything) and use another to do it twice and drop the 
/// lowest result. 
pub struct PoolRoller {
    count: u8,
    die: Rc<dyn SubRoller>,
    strategy: Strategy,
}
impl PoolRoller {
    /// If the strategy drops the kept rolls to 0 or less, returns None
    /// Otherwise, it returns a PoolRoller with the given values
    /// Note: this allows you to use Strategies that have an order_by that may not be
    /// effective with the given "die", including an empty one. In cases where it has
    /// no effect, DropLowest will simply drop the earliest roll(s) and DropHighest will
    /// simply drop the latest roll(s)
    pub fn new(die: Rc<dyn SubRoller>, count: u8, strategy: Strategy) -> Option<Rc<Self>> {
        if strategy.count() >= count { None }
        else { Some(Rc::new(Self { count, die, strategy })) } }

    pub fn basic(die: Rc<dyn SubRoller>, count: u8) -> Rc<Self> {
        Rc::new (Self { count, die, strategy: Strategy::KeepAll }) }

    pub fn better_of(die: Rc<dyn SubRoller>, order_by: Vec<Rc<dyn Unit>>) -> Rc<Self> {
        Rc::new(Self {
            count: 2,
            die,
            strategy: Strategy::DropLowest{count: 1, order_by} }) }
}
impl Roller for PoolRoller {
    fn description(&self) -> String {
        if self.die.is_die() { 
            format!("{}{}{}", self.count, self.die.description(), self.strategy.descriptor()) }
        else { 
            format!("{}({}){}", self.count, self.die.description(), self.strategy.descriptor()) } }

    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        let mut rolls: Vec<Box<dyn SubRoll>> = Vec::with_capacity(self.count as usize);
        for _ in 0..self.count {
            rolls.push(self.die.clone().inner_roll_with(rng));
        }
        match self.strategy {
            KeepAll => {
                PoolRoll::new(
                    rolls,
                    Vec::with_capacity(0) ) }
            DropLowest { count, order_by: _ } => {
                let cut_idx = count as usize;
                rolls.sort_by(|a, b| self.strategy.order_comparator(a, b));
                let kept = rolls.split_off(cut_idx);
                PoolRoll::new(
                    kept,
                    rolls ) }
            DropHighest { count, order_by: _ } => {
                let cut_idx = rolls.len() - (count as usize);
                rolls.sort_by(|a, b| self.strategy.order_comparator(a, b));
                let dropped = rolls.split_off(cut_idx);
                PoolRoll::new(
                    rolls,
                    dropped ) } } }
}
impl SubRoller for PoolRoller {
    fn is_simple(&self) -> bool { self.die.is_simple() && self.strategy.is_simple() }
    
    fn inner_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        let mut rolls: Vec<Box<dyn SubRoll>> = Vec::with_capacity(self.count as usize);
        for _ in 0..self.count {
            rolls.push(self.die.clone().inner_roll_with(rng));
        }
        match self.strategy {
            KeepAll => {
                PoolRoll::new(
                    rolls,
                    Vec::with_capacity(0) ) }
            DropLowest { count, order_by: _ } => {
                let cut_idx = count as usize;
                rolls.sort_by(|a, b| self.strategy.order_comparator(a, b));
                let kept = rolls.split_off(cut_idx);
                PoolRoll::new(
                    kept,
                    rolls ) }
            DropHighest { count, order_by: _ } => {
                let cut_idx = rolls.len() - (count as usize);
                rolls.sort_by(|a, b| self.strategy.order_comparator(a, b));
                let dropped = rolls.split_off(cut_idx);
                PoolRoll::new(
                    rolls,
                    dropped ) } } }
}


struct PoolRoll {
    kept_rolls: Vec<Box<dyn SubRoll>>,
    dropped_rolls: Vec<Box< dyn SubRoll>>,
}
impl PoolRoll {
    fn new(kept_rolls: Vec<Box<dyn SubRoll>>, dropped_rolls: Vec<Box<dyn SubRoll>>) -> Box<Self> {
        Box::new(Self {kept_rolls, dropped_rolls}) }

    fn build_intermediate_results_part(rolls: &Vec<Box<dyn SubRoll>>, separator: &str) -> String {
        rolls.iter()
            .map(|roll| wrapped_text(&roll.intermediate_results(), roll.is_simple()))
            .collect::<Vec<String>>()
            .join(separator) }
}
impl Roll for PoolRoll {
    fn intermediate_results(&self) -> String {
        if self.is_simple() {
            Self::build_intermediate_results_part(&self.kept_rolls, " + ") }
        else {
            format!(
                "{}, [dropped: {}]",
                Self::build_intermediate_results_part(&self.kept_rolls, " + "),
                Self::build_intermediate_results_part(&self.dropped_rolls, ", ") ) } }
    
    fn final_result(&self) -> String { self.totals().to_string() }
}
impl SubRoll for PoolRoll {
    fn is_simple(&self) -> bool { self.dropped_rolls.len() == 0 }

    /// Returns the rolled faces of just the kept rolls
    fn rolled_faces(&self) -> Vec<&DieRoll> {
        let mut out = Vec::with_capacity(self.kept_rolls.len());
        for roll in self.kept_rolls.iter() {
            out.append(roll.rolled_faces().as_mut()) }
        out }

    /// Returns the totals of just the kept rolls
    fn totals(&self) -> Values {
        let mut out = Values::new();
        for roll in self.kept_rolls.iter() {
            out.add_all_values(roll.totals()); }
        out }
}