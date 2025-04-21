use std::{
    fmt::{Display, Error, Formatter},
    rc::Rc
};
use crate::{
    {Value, Values},
    random::{choose_from, Rng, default_rng} };


/// `Die`/Dice are the most obvious inclusion in a dice-rolling program. 
/// They're quite simple, with a name and a `Vec` of reference-counted `Face`s.
/// The more complicated part is the collection of values in `Face`s. 
#[derive(Clone, Debug)]
pub struct Die {
    pub name: String,
    pub faces: Vec<Rc<Face>>,
}
impl  Die {
    pub fn new(name: String, faces: Vec<Rc<Face>>) -> Rc<Self> {
        Rc::new(Self { name, faces }) }

    /// "Roll" the `Die` and see which `Face` is up. Accepts a random number
    /// generator (`crate::random::Rng`) as well, allowing for customizable
    /// seeds for reproducibility as needed. If you don't want to bother with
    /// providing an `Rng`, use the `roll()` method instead.
    pub fn roll_face_with(&self, mut rng: Rng) -> Rc<Face> {
        choose_from(&self.faces, &mut rng) }
    
    /// "Roll" the `Die` and see which `Face` is up. Uses the default random
    /// number generator given by `crate::random::default_rng()`.
    pub fn roll_face(&self) -> Rc<Face> {
        self.roll_face_with(default_rng()) }
}
impl Display for Die {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&self.name) }
}


/// A `Face` of a `Die`. Has a label in order to have a short bit of text for
/// display purposes.
#[derive(Clone, Debug)]
pub struct Face {
    pub label: String,
    pub values: Values,
}
impl Face {
    pub fn new(label: String, values: Vec<Value>) -> Rc<Self> {
        Rc::new(Face{ label, values: Values::from(values) }) }
}
impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&self.label) }
}
