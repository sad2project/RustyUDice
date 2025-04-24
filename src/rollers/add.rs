use crate::rollers::Roller;

pub struct AddRoller {
    lhs: Rc<dyn Roller>,
    rhs: Rc<dyn Roller>
}
impl AddRoller {
    pub fn new(lhs: Rc<dyn Roller>, rhs: Rc<dyn Roller>) -> Rc<Self> {
        Rc::new(Self{ lhs, rhs })
    }
}
impl Roller for AddRoller {
    todo!()
}
impl ComposableRoll for AddRoller {
    todo!()
}