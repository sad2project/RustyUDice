// This can't be a proper `Roller`. It doesn't come up with a single result total. 
// It's used for rolling multiple things at once but not adding them up. 
pub struct MultiRoller;
impl MultiRoller {
    pub fn new() -> Box<Self> { Box::new(MultiRoller) }
}