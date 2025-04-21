mod add;
mod die;
mod multi;
mod modifier;
mod pool;
mod stats;
mod subtract;

pub use add::*;
pub use die::*;
pub use multi::*;
pub use modifier::*;
pub use pool::*;
pub use stats::*;
pub use subtract::*;

use std::{
    rc::Rc,
    vec::Vec };
use crate::{
    Value, Values,
    random::{default_rng, Rng} };

pub trait Roller {
    fn is_simple(&self) -> bool;
    fn description(&self) -> String;
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll>;
    fn roll(self: Rc<Self>) -> Box<dyn Roll> { self.roll_with(default_rng()) }
}
  

pub trait Roll {
    fn is_simple(&self) -> bool;
    fn rolled_faces(&self) -> Vec<&DieRoll>;
    fn totals(&self) -> Values;
    fn intermediate_results(&self) -> String;
    fn final_result(&self) -> String { self.totals().to_string() }
}