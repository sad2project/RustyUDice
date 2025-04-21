use crate::rollers::Roller;

pub struct SubtractRoller {
    lhs: Box<dyn Roller>,
    rhs: Box<dyn Roller>,
}
impl SubtractRoller {
    pub fn new(lhs: Box<dyn Roller>, rhs: Box<dyn Roller>) -> Box<Self> {
        Box::new(Self{ lhs, rhs })
    }
}