/// `Roller`s are, as you can probably guess, how you roll dice. True, `Die` has its own
/// `roll*()` methods, but those are actually meant to supplement `Roller`s. You can use
/// them separately, but they won't give you the power and flexibility that `Roller`s 
/// will. There are several kinds of `Roller`s that have different effects, and they're
/// composable with each other, genrerally. Specifically `ComposableRoller`s are
/// composable, and the others can be composed of, but not compose, other `Roller`s.
///
/// What are these special "effects" that `Roller`s give us? You should look into them
/// to see all the things they can do but, we'll mention some just to give you an idea 
/// here. First off, there's the `PoolRoller`, which rolls the composed `Roller` (probably
/// just a single die, but it's not limited to that) several times and adds all the 
/// `Roll`s up. You can even tell it to drop (not count) some of the highest or lowest
/// `Roll`s. Another `Roller`, though pretty basic, is the `ModifierRoller`, which will
/// add a `Value`onto the `Roll` from an inner `Roller`. Lastly, I'll mention the 
/// StatisticsRoller, which takes another `Roller` and rolls it a number of times that you
/// tell it to, and it derives statistics from it, namely the average, median, mode, and 
/// standard deviation. It also collects every rolled `Value` from the test so you can
/// do your own statistics on it, if you'd like.
///
/// I've mentioned `Roll`s a few times now. What are they? Technically, they're the real
/// powerhouses here. The `Roller`s just generate the random die faces, but the `Rolls`
/// collect those and bring it all together into useful information. From the `Roll`, you
/// can describe the entirety of the roll ("this die rolled a 7, and this one rolled a 2,
/// and 4 was added on after"; that kind of stuff, though more strictly defined than that),
/// as well as calculating the end total(s).
mod die;
mod math;
mod multi;
mod modifier;
mod pool;
mod stats;

pub use self::{
    die::*,
    math::*,
    multi::*,
    modifier::*,
    pool::*,
    stats::* };

use std::{
    rc::Rc,
    vec::Vec };
use crate::{
    Value, Values,
    random::{default_rng, Rng} };


/// `Roller` is the basis of how dice are rolled. `Die` itself is a `Roller`, though it has
/// its own `roll*()` methods that aren't part of the `Roller` trait, which only produce a
/// random `Face` from the `Die`. `Roller`s are used to gather multiple dice or add on 
/// modifiers, etc., in order to make more interesting rolls and gather it all into `Roll`
/// objects. 
///
/// Generally, `Roller`s are composable, building on top of one another, but if a type 
/// implements `Roller` but not `ComposableRoller`, it cannot be composed inside another
/// Roller. These `Roller`s are `MultiRoller` and `StatisticsRoller`, which produce multiple
/// results that aren't able to be made into a single one to be modified. If you do want to
/// modify the results, you need to put that modification on all the inner `Roller`s
///
/// #Implementing a `Roller`
/// ##`Roller` or `ComposableRoller`?
/// First things first: is your roller just a `Roller` or is it a `ComposableRoller`? The
/// tell is whether your roller produces just a single total or multiple. There are currently
/// only two plain `Roller`s: `MultiRoller` and `StatisticsRoller`. `MultiRoller` is meant for
/// making multiple, completely different rolls, such as to do an attack and damage roll all
/// at once. `StatisticsRoller` makes the same roll over and over again in order to calculate
/// the average, median, mode, and standard deviation for that roll. In neither case is there
/// an obvious single roll to hand up to a wrapper `Roller` to modify. But if your roller is
/// meant to produce a single total, it should implement `ComposableRoller` (which is an
/// extension of `Roller`, so you'll have to implement both). Implementing `ComposableRoller`
/// will be covered in the documentation of that trait.
///
/// ##Wrapping Other `Roller`s
/// The next thing to know about implementing your own roller is that, if it wraps (an)other
/// roller(s), the type of that field should be `Rc<dyn ComposableRoller>`. In general, there
/// will only ever be a single "instance" of a particular roller, but using `Rc` makes it just
/// a little easier to not have to think about lifetimes or cloning major structs. And using
/// `ComposableRoller` over `Roller` is because if it's not composable, it's not meant to be
/// wrapped, as mentioned earlier.
///
/// ##Implementing `description()`
/// Most likely, you're going to want to use description of your inner roller(s) as part of
/// the description. If so, prefer `inner_description()` of the inner roller(s) over
/// `description()`. It will wrap the text in parentheses if it thinks it will be clearer for
/// doing so. If you think that's never necessary, you may simply use `description()`.
///
/// ##Implementing `roll_with()`
/// When delegating to the rolling of inner rollers, you should always pass the `Rng` on
/// down via the inner roller's `composable_roll()` (from `ComposableRoller`). This will 
/// ensure that your returned `Roll` is composed of the correct `ComposableRoll`s
///
/// Don't override `roll()`; it creates a default `Rng` and calls `roll_with()`.
pub trait Roller {
    /// Returns a `String` that describes what the roller rolls. i.e. "2d8 + 6"
    fn description(&self) -> String;

    /// "Rolls" the dice using the given random number generator and produces a `Roll`
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll>;

    /// "Rolls" the dice using the default random number generator and produces a `Roll`
    /// Do not override
    fn roll(self: Rc<Self>) -> Box<dyn Roll> { self.roll_with(default_rng()) }
}


/// `ComposableRoller`s are `Roller`s that only produce a single result, so that result
/// can get modified by wrapping it in another `Roller`. All of its additional methods are
/// meant to be called by a wrapping `Roller` or by default implementations of other methods.
///
/// #Implementing `ComposableRoller`
/// ## Implementing `is_simple()`
/// This method is for determining whether it's necessary to put parentheses around the 
/// description when is part of a wrapper roller's description. It is then used by 
/// `inner_description()`, which will wrap the description as needed. If you are having
/// a difficult time deciding whether to return `true` or `false`, most likely, that means
/// it is dependent on whether the roller you wrap is simple or not. 
///
/// ## Implementing `composable_roll()`
/// This should be written exactly the same as `roll_with()` on `Roller`, except that you
/// may need to do a downcast. Most likely, though, it will be an automatic cast from the
/// static type to the dynamic type. 
pub trait SubRoller: Roller where Self: 'static {
    /// When generating a description (using `description()`), you want to know whether
    /// it's necessary to wrap the text description of an inner `Roller` with parentheses
    /// to make it clearer and easier to understand. But you don't want unnecessary
    /// parentheses, which can actually get in the way of clarity. For that reason, we 
    /// have `is_simple()`, which, if it returns `false`, lets the wrapper know the 
    /// description should be wrapped in parentheses. Used by `inner_description()`.
    fn is_simple(&self) -> bool;

    /// This is just for the `Die``Roller` implementation to say `true` to, so just leave
    /// it as false
    fn is_die(&self) -> bool { false }

    /// Uses `is_simple()` to determine whether to wrap `description()` in parentheses.
    /// Unless you possibly want to wrap it in something else, do not implement this 
    /// yourself.
    fn inner_description(&self) -> String {
        if self.is_simple() { self.description() }
        else { format!("({})", self.description() ) } }

    /// The same as `roll_with()`, but returns a `ComposableRoll` to avoid needing to
    /// upcast or downcast. When a wrapper `Roller` calls a roll method of an inner 
    /// `Roller`, it should be this one.
    fn inner_roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn SubRoll>;

    fn n_times(self: Rc<Self>, n: u8) -> Rc<PoolRoller> where Self: Sized {
        PoolRoller::basic(self.clone(), n) }

    fn n_times_and(self: Rc<Self>, n: u8, strategy: Strategy) -> Option<Rc<PoolRoller>> where Self: Sized {
        PoolRoller::new(self, n, strategy) }

    fn plus(self: Rc<Self>, other: Rc<dyn SubRoller>) -> Rc<MathRoller> where Self: Sized {
        MathRoller::add(self, other) }

    fn minus(self: Rc<Self>, other: Rc<dyn SubRoller>) -> Rc<MathRoller> where Self: Sized {
        MathRoller::subtract(self, other) }

    fn plus_modifier(self: Rc<Self>, value: Values) -> Rc<MathRoller> where Self: Sized {
        MathRoller::add(self, value.as_roller()) }

    fn minus_modifier(self: Rc<Self>, value: Values) -> Rc<MathRoller> where Self: Sized {
        MathRoller::subtract(self, value.as_roller()) }
    
    fn plus_named_modifier(self: Rc<Self>, name: Name, values: Values) -> Rc<MathRoller> where Self: Sized {
        MathRoller::add(self, values.as_roller_with_name(name)) }
        
    fn minus_named_modifier(self: Rc<Self>, name: Name, values: Values) -> Rc<MathRoller> where Self: Sized {
        MathRoller::subtract(self, values.as_roller_with_name(name)) }

    fn get_stats(self: Rc<Self>, num_runs: u32) -> Rc<StatsRoller> where Self: Sized {
        StatsRoller::new(self, num_runs) }
}


/// `Roll`s have the same dual nature of `Roller`s, being either plain `Roll`s or 
/// `ComposableRoll`s. If you make a roller that implements `ComposableRoller`, then the
/// roll type it returns better return `ComposableRoll`. And if the roller only implements
/// `Roller`, then the roll it returns better only implement `Roll`. 
///
/// #Implementing `Roll`
/// ##Wrapping Other `Roll`s
/// Just like with `Roller`s, inner `Roll`s should all be `ComposableRoll`s, specifically
/// `Rc<dyn ComposableRoll>`. 
///
/// ## Implementing `intermediate_results()`
/// This method is meant to show all the dice that were rolled and how they're combined together.
/// To get that, you piece together the `composable_intermediate_results()` of all the wrapped 
/// `Roll`s with some text between/around to show what your `Roller` is doing to modify it. In some 
/// cases,  doing all that text is impractical, such as for the `StatisticsRoller`, since it 
/// generally rolls the dice so many times that it would overflow the text area that it's meant for. 
/// In that case, try to be as descriptive as possible. For example, `StatisticsRoll` simply says
/// "Rolled X times".
///
/// ##Implementing `final_result()`
/// This method is meant to show the total of all the values rolled with all their modifications.
/// if this is a `ComposableRoll`, it's incredibly easy. Just type `self.totals().to_string()`.
///
/// If it's not composable, it's still pretty easy; you take the `totals()` `Values` from each of 
/// the wrapped `ComposableRoll`s, combine them using the add or subtract methods on them, then use `Values`'
/// `to_string()` method.
///
/// Again, you may have an exceptional case (typically if it's not a `ComposableRoll`). But that's
/// the general way to implement it.
pub trait Roll {
    /// Returns a `String` that lays out all the dice rolls and how they were combined together
    fn intermediate_results(&self) -> String;

    /// Returns a `String` that summarizes the total of all the rolls and how they're combined together.
    fn final_result(&self) -> String;
}


/// `ComposableRoll` is to `Roll` as `ComposableRoller` is to `Roller`. All `ComposableRoll`s should
/// come from `ComposableRoller`s, and all plain `Roll`s should come from plain `Roller`s. Just as
/// the name implies, `ComposableRoll`s are meant to be able to be composed into other `Roll`s, 
/// whereas, if a `Roll` doesn't implement `ComposableRoll`, then it isn't meant to be wrapped by
/// other `Roll`s.
///
/// #Implementing `ComposableRoll`
/// ## Implementing `is_simple()`
/// In 90+% of cases, this should be determined the same way as the `Roller` that creates it, but 
/// there may be some circumstances where the output of `intermediate_result()` should be wrapped 
/// in parentheses when the `Roller`'s `description()` shouldn't, such as when a `description()`
/// has a shorthand that doesn't apply to `intermediate_result()` (e.g. 20d6 vs all 20 of the rolls
/// being listed out).
///
/// ##Implementing `rolled_faces()`
/// This is an easy one; get the `rolled_faces()` from every wrapped `Roll` and combine them into 
/// a single Vector. It could be made with a default implementation, but that would require a new 
/// method for you to implement that returns all the wrapped `Roll`s. That would be easier for you,
/// but it would bloat the code with otherwise unused methods, and it's not that difficult to do now.
/// Keep in mind, there's a chance this method is also going to be removed.
///
/// ##Implementing `totals()`
/// Using the `Values` objects from inner `Roll`s' `totals()` methods, combine them and add and 
/// subtract from them as dictated by this `Roll`'s effects, then return the resulting `Values`
pub trait SubRoll: Roll {
    /// Used to determine whether `intermediate_results()` calls of wrapper `Roll`s should wrap your
    /// `intermediate_results()` call in parentheses to make the output clearer and easier to read.
    /// this decision is made automatically by `composable_intermediate_results()` using the return
    /// value of this method.
    fn is_simple(&self) -> bool;

    /// Do not override. This is the composable version of `intermediate_results()`, which uses
    /// `is_simple()` to determine whether to wrap the text in parentheses.
    fn inner_intermediate_results(&self) -> String {
        if self.is_simple() { self.intermediate_results() }
        else { format!("({})", self.intermediate_results()) } }

    /// Returns all the `DieRoll`s that make up this total `Roll`. 
    fn rolled_faces(&self) -> Vec<&DieRoll>;

    /// Returns the final total of all the rolls combined.
    fn totals(&self) -> Values;
}