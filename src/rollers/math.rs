use std::{
    rc::Rc
};

use crate::{random::Rng, rollers::{Roller, Roll, SubRoller, SubRoll}, Values};

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
            MatherT::First(roller) => MathT::First(roller.clone().inner_roll_with(rng)),
            MatherT::Add(roller) => MathT::Add(roller.clone().inner_roll_with(rng)),
            MatherT::Subtract(roller) => MathT::Subtract(roller.clone().inner_roll_with(rng)) } }
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
        self.inner.push(MatherT::Add(roller));
        self }
    
    pub fn plus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(MatherT::Add));
        self }
    
    pub fn minus(mut self, roller: Rc<dyn SubRoller>) -> Self {
        self.inner.push(MatherT::Subtract(roller));
        self }
    
    pub fn minus_all(mut self, rollers: impl IntoIterator<Item=Rc<dyn SubRoller>>) -> Self {
        self.inner.extend(rollers.into_iter().map(MatherT::Subtract));
        self }
}
impl Roller for MathRoller {
    fn description(&self) -> String {
        self.inner.iter()
            .map(MathRollerType::description)
            .collect::<Vec<String>>()
            .join("") }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        MathRoll::new(self.inner.iter().map(|roller| roller.roll_with(rng))) }
}
impl SubRoller for MathRoller {
    fn is_simple(&self) -> bool { false }

    fn inner_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll> {
        MathRoll::new(self.inner.iter().map(|roller| roller.roll_with(rng))) }
}

enum MathRollType {
    First(Box<dyn SubRoll>),
    Add(Box<dyn SubRoll>),
    Subtract(Box<dyn SubRoll>)
}
use MathRollType as MathT;
use crate::rollers::DieRoll;

impl MathRollType {
    fn intermediate_results(&self) -> String {
        match self {
            MathT::First(roll) => roll.inner_intermediate_results(),
            MathT::Add(roll) => format!(" + {}", roll.inner_intermediate_results()),
            MathT::Subtract(roll) => format!(" - {}", roll.inner_intermediate_results()) } }
    
    fn rolled_faces(&self) -> Vec<&DieRoll> {
        match self {
            MathT::First(roll) 
            | MathT::Add(roll) 
            | MathT::Subtract(roll) => roll.rolled_faces() } }
    
    fn totals(&self) -> Values {
        match self {
            MathT:: First(roll) | MathT::Add(roll) => roll.totals(),
            MathT::Subtract(roll) => -roll.totals() } }
}

struct MathRoll {
    inner: Vec<MathRollType>
}
impl MathRoll {
    pub fn new(inner: impl IntoIterator<Item=MathRollType>) -> Box<MathRoll> {
        Box::new(MathRoll { inner: inner.into_iter().collect() }) }
}
impl Roll for MathRoll {
    fn intermediate_results(&self) -> String {
        self.inner.iter()
            .map(MathRollType::intermediate_results)
            .collect() }

    fn final_result(&self) -> String {
        self.totals().to_string() }
}
impl SubRoll for MathRoll {
    fn is_simple(&self) -> bool { false }

    fn rolled_faces(&self) -> Vec<&DieRoll> {
        let mut output = Vec::with_capacity(self.inner.len());
        for inner_roll in self.inner.iter() {
            output.extend(inner_roll.rolled_faces()); }
        output }

    fn totals(&self) -> Values {
        self.inner.iter()
            .map(MathRollType::totals)
            .collect::<Values>() }
}