use std::{
    fmt::{Debug, Display, Error, Formatter},
    rc::Rc };
use crate::{
    Unit,
    u64_gen::get_u64 };

pub mod tiered;
pub use tiered::{TieredUnit, Tier};


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
        f.write_str("Default Numeric") }
}


#[derive(Debug)]
pub struct BasicUnit {
    id: u64,
    name: String,
    output_format: String,
    ignore_zero: bool,
}
impl BasicUnit {
    pub fn new(name: String, output_format: String, ignore_zero: bool) -> Rc<Self> { 
        Rc::new(Self { 
            id: get_u64(), 
            name,
            output_format, 
            ignore_zero}) }
    
    pub fn rebuild(id: u64, name: String, output_format: String, ignore_zero: bool) -> Rc<Self> { 
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
        f.write_fmt(format_args!("{}", self.name)) }
}
