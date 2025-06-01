use std::{
    fmt::{Display, Error, Formatter},
    ops::RangeInclusive,
    rc::Rc };
use crate:: {
    Name, Unit,
    random::new_id };


/// When the range of values changes how you'd give the output, you need a TieredUnit.
/// For example, you might want and total of 0 or less to be "failure" and anything higher to be
/// "X successes". To accomplish this, you need a TieredUnit with 2 Ranges; one that goes
/// from the minimum value to 0 (ranges are fully inclusive) with an output_format of "failure",
/// and the other that goes from 1 to the maximum value with an output_format of "{} successes".
/// You could even insert a Range that just covers 1 with an output_format of "1 success" (there's
/// no point in formatting out the 1 when it will always say 1.
/// If you want the absolute value of the number (e.g. "2 failures" instead of "-2 failures"), put
/// a pipe ("|") inside the brackets.
#[derive(Debug)]
pub struct TieredUnit {
    id: u64,
    name: Name,
    tiers: Vec<Tier>,
}
impl TieredUnit {
    pub fn new(name: Name, tiers: impl Into<Vec<Tier>>) -> Rc<Self> {
        Rc::new(Self { 
            id: new_id(), 
            name,
            tiers: tiers.into() }) }
            
    pub fn pos_zero_neg(name: Name, pos_fmt: &str, zero_fmt: &str, neg_fmt: &str) -> Rc<Self> {
        Rc::new(Self {
            id: new_id(),
            name,
            tiers: vec![
                Tier{ range: i32::MIN..=-1, output_format: neg_fmt.into() },
                Tier{ range: 0..=0, output_format: zero_fmt.into() },
                Tier{ range: 1..=i32::MAX, output_format: pos_fmt.into() }] }) }
                
    pub fn pos_neg(name: Name, pos_fmt: &str, neg_fmt: &str) -> Rc<Self> {
        Rc::new(Self {
            id: new_id(),
            name,
            tiers: vec![
               Tier{ range: i32::MIN..=-1, output_format: neg_fmt.into() },
               Tier{ range: 1..=i32::MAX, output_format: pos_fmt.into() }] }) }
    
    pub fn rebuild(id: u64, name: Name, tiers: impl Into<Vec<Tier>>) -> Rc<Self> { 
        Rc::new(Self { 
            id,
            name,
            tiers: tiers.into() }) }
}
impl Unit for TieredUnit {
    fn id(&self) -> u64 { self.id }
    
    fn output_for(&self, total: i32) -> String {
        for tier in &self.tiers {
            if tier.contains(total) {
                return tier.output_for(total) } }
        "".to_string() }
}
impl Display for TieredUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("{}", self.name)) }
}


#[derive(Debug)]
pub struct Tier {
    pub range: RangeInclusive<i32>,
    pub output_format: String
}
impl Tier {
    pub fn contains(&self, total: i32) -> bool { self.range.contains(&total) }
    
    pub fn output_for(&self, total: i32) -> String {
        self.output_format.replace("{|}", &total.abs().to_string()).replace("{}",  &total.to_string()) }
}
