use std::rc::Rc;
use crate::dice::Die;
use crate::Unit;

pub struct SetId(pub u64);


pub trait DieStorage {
    fn get_all_dice(&self) -> Vec<Rc<Die>>;
    fn get_set_dice(&self, set: SetId) -> Vec<Rc<Die>>;
    fn get_die(&self, set: SetId, die_name: String) -> Option<Rc<Die>>;
    
    fn get_all_units(&self, ) -> Vec<Rc<dyn Unit>>;
    fn get_set_units(&self, set: SetId) -> Vec<Rc<dyn Unit>>;
    fn get_unit(&self, set: SetId, unit_name: String) -> Option<Rc<dyn Unit>>;
    
    fn store_dice(&self, dice: Vec<Rc<Die>>) -> SetId;
}