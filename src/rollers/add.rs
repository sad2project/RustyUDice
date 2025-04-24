use crate::rollers::Roller;

pub struct AddRoller {
    lhs: Rc<dyn ComposableRoller>,
    rhs: Rc<dyn ComposableRoller>
}
impl AddRoller {
    pub fn new(lhs: Rc<dyn ComposableRoller>, rhs: Rc<dyn ComposableRoller>) -> Rc<Self> {
        Rc::new(Self{ lhs, rhs })
    }
}
impl Roller for AddRoller {
    fn description(&self) -> String {
        ; }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Rc<dyn Roll> {
        ; }
}
impl ComposableRoll for AddRoller {
    fn is_simple(&self) -> bool
}