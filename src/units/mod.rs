use std::{
    fmt::{Debug, Display, Error, Formatter},
    rc::Rc };
use crate::{
    Name, Unit,
    random::new_id };

pub mod tiered;
pub use tiered::{TieredUnit, Tier};


/// The `Unit` intended for your typical numeric dice. It doesn't provide any kind of label with the
/// total; it just outputs the numeric total itself. 
pub struct DNumUnit;
impl DNumUnit {
    pub fn new() -> Rc<Self> { Rc::new(DNumUnit) }
}
impl Unit for DNumUnit {
    fn id(&self) -> u64 { 0 }
    
    fn output_for(&self, total: i32) -> String { total.to_string() }
}
impl Display for DNumUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("Total") }
}
impl Debug for DNumUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("Default Numeric Unit") }
}


/// A basic `Unit` type that only has one possible output format.
///
/// The format will replace a "{}" with the total or a "{|}" with the absolute value of the total.
/// If you don't want there to be any output for the `Unit` if the total is zero, you can set
/// `ignore_zero` to `true`.
#[derive(Debug)]
pub struct BasicUnit {
    id: u64,
    name: Name,
    output_format: String,
    ignore_zero: bool,
}
impl BasicUnit {
    pub fn new(name: &str, output_format: &str, ignore_zero: bool) -> Rc<Self> { 
        Rc::new(Self { 
            id: new_id(), 
            name: name.try_into().unwrap(),  // TODO: Remove the unwrap(). Pass up the Result or ask for Name
            output_format: output_format.into(), 
            ignore_zero}) }
    
    pub fn rebuild(id: u64, name: Name, output_format: String, ignore_zero: bool) -> Rc<Self> { 
        Rc::new(Self { 
            id, 
            name,
            output_format, 
            ignore_zero}) }
}
impl Unit for BasicUnit {
    fn id(&self) -> u64 { self.id }
    
    fn output_for(&self, total: i32) -> String {
        if total == 0 && self.ignore_zero { return "".to_string() }
        self.output_format
            .replace("{|}", &total.abs().to_string())
            .replace("{}", &total.to_string()) }
}
impl Display for BasicUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.name) }
}
