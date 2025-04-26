use std::{
    rc::Rc
};

use crate::{
    random::Rng, 
    rollers::{Roller, Roll, SubRoller, SubRoll}};

enum MathRollerType {
    First(Rc<dyn SubRoller>),
    Add(Rc<dyn SubRoller>),
    Subtract(Rc<dyn SubRoller>)
}
use MathRollerType as MatherT;
impl MathRollerType {
    fn description(&self) -> String {
        match self {
            MatherT::First(roller) => roller.inner_description(),
            MatherT::Add(roller) => format!(" + {}", roller.inner_description()),
            MatherT::Subtract(roller) => format!(" - {}", roller.inner_description()) } }
    
    fn roll_with(&self, rng: Rng) -> MathRollType {
        match self {
            MatherT::First(roller) => MathT::First(roller.roll_sub_with(rng)),
            MatherT::Add(roller) => MathT::Add(roller.roll_sub_with(rng)),
            MatherT::Subtract(roller) => MathT::Subtract(roller.roll_sub_with(rng)) } }
}

pub struct MathRoller {
    inner: Vec<MathRollerType>
}
impl MathRoller {
    pub fn add(lhs: Rc<dyn SubRoller>, rhs: Rc<dyn SubRoller>) -> Rc<Self> {
        Rc::new(Self{ inner: vec![MatherT::First(lhs), MatherT::Add(rhs)] }) }
    
    pub fn subtract(lhs: Rc<dyn SubRoller>, rhs: Rc<dyn SubRoller>) -> Rc<Self> {
        Rc::new(Self{ inner: vec![MatherT::First(lhs), MatherT::Subtract(rhs)] }) }
    
    pub fn plus(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(Add(roller));
        self }
    
    pub fn plus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(Add));
        self }
    
    pub fn minus(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(Subtract(roller));
        self }
    
    pub fn minus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(Subtract));
        self }
}
impl Roller for MathRoller {
    fn description(&self) -> String {
        ; }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        ; }
}
impl SubRoller for MathRoller {
    fn is_simple(&self) -> bool { false }

    fn sub_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        todo!()
    }
}

enum MathRollType {
    First(Box<dyn SubRoll>),
    Add(Box<dyn SubRoll>),
    Subtract(Box<dyn SubRoll>)
}
use MathRollType as MathT;
impl MathRollType {
    fn intermediate_results(&self) -> String {
        match self {
            First(roll) => roll.composable_intermediate_results(),
            Add(roll) => format!(" + {}", roll.composable_intermediate_results()),
            Subtract(roll) => format!(" - {}", roll.composablt_intermediate_results()) } }
    
    fn rolled_faces(&self) -> Vec<&DieRoll> {
        match self {
            First(roll) | Add(roll) | Subtract(roll) => roll.rolled_faces() } }
    
    fn totals(&self) -> Values {
        match self {
           First(roll) | Add(roll) => roll.totals(),
           Subtract(roll) => -roll.totals() } }
}