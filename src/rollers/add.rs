use crate::rollers::Roller;

pub struct AddRoller {
    lhs: Box<dyn Roller>,
    rhs: Box<dyn Roller>
}
impl AddRoller {
    pub fn new(lhs: Box<dyn Roller>, rhs: Box<dyn Roller>) -> Box<Self> {
        Box::new(Self{ lhs, rhs })
    }
}