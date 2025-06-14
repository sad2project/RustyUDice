mod calculate;
use self::calculate::*;

use std::{
    fmt::{Display, Error, Formatter},
    num::NonZero,
    ops::Deref,
    rc::Rc };
use crate::{
    {Unit, Values},
    random::{Rng, default_rng},
    rollers::{Roller, Roll, SubRoll, SubRoller} };

/// `StatsRoller` is used to find out the statistics of a
/// roll. It'll perform the given `Roller`'s roll a number of times
/// given, then provide the average, median, mode, and standard 
/// deviation for each `Unit`. You can also extract the
/// results of each individual roll and calculate any other desired
/// statistics based on that, such as graphing the results out somehow.
///
/// The above listed stats are put out in a string using `Roll`'s 
/// output() method, but you can also just use the individual methods on 
/// `StatisticsRoll`, `average()`, `median()`, `mode()`, and 
/// `std_deviation()`, which will return `Stat`s.
/// 
/// Unfortunately, since `Unit`s work with integer numbers and the
/// stats use floating point numbers (though, median and mode don't NEED to;
/// they're simply kept consistent with the others that do), we can't convert
/// the numbers into the `Unit`'s output. It will simply have the
/// `Unit` and the value printed side-by-side.
pub struct StatsRoller {
    runs: u32,
    roller: Rc<dyn SubRoller>,
}
impl StatsRoller {
    /// Creates a new `StatsRoller` using the given roller and a number of times to run it in order
    /// to generate the statistics
    pub fn new(roller: Rc<dyn SubRoller>, num_runs: NonZero<u32>) -> Rc<Self> {
        Rc::new(Self { runs: num_runs.get(), roller }) }
  
    /// Does the same thing as `roll()`, except it returns the roller as a statically-typed
    /// `StatisticsRoll` instead of a `dyn Roll`, giving access to its extra methods
    pub fn statistics_roll(self: Rc<Self>) -> Box<StatisticsRoll> { 
        self.statistics_roll_with(default_rng()) }
    
    /// Does the same thing as `roll_with()`, except it returns the roller as a statically-typed
    /// `StatisticsRoll` instead of a `dyn Roll`, giving access to its extra methods
    pub fn statistics_roll_with(self: Rc<Self>, rng: Rng) -> Box<StatisticsRoll> {
        StatisticsRoll::new(
            (0..self.runs)
            .map(|_| self.roller.clone().inner_roll_with(rng.clone()))
            .collect()) }
}
impl Roller for StatsRoller {
    fn description(&self) -> String {
        format!("Runs '{}' {} times", self.roller.description(), self.runs) }
    
    fn roll_with(self: Rc<Self>, rng: Rng) -> Box<dyn Roll> {
        self.statistics_roll_with(rng) }
}


/// `StatisticsRoll` is a `Roll` that is created by `StatsRoller` that gathers up all the data from
/// numerous rolls and calculates the statistics of them.
pub struct StatisticsRoll {
    rolls: Vec<Box<dyn SubRoll>>,
    collected_stats: CollectedStats
}
impl StatisticsRoll {
    fn new(rolls: Vec<Box<dyn SubRoll>>) -> Box<Self> {
        let collected_stats = StatisticsRoll::run_calcs(&rolls);
        Box::new(Self{ rolls, collected_stats }) }
    
    fn run_calcs(rolls: &Vec<Box<dyn SubRoll>>) -> CollectedStats{
        let roll_vals: Vec<Values> = rolls.iter()
            .map(|roll| roll.totals())
            .collect();
        let builder = CollectedStatsBuilder::new(roll_vals);
        builder.build() }
    
    /// Look up the statistics for the given `Unit`, if there are any
    pub fn stats_for(&self, unit: Rc<dyn Unit>) -> Option<&UnitStats> {
        self.collected_stats.for_unit(unit) }
    
    /// Returns the average for each `Unit`
    pub fn averages(&self) -> Stat { self.collected_stats.averages() }
    
    /// Returns the median for each `Unit`
    pub fn medians(&self) -> Stat { self.collected_stats.medians() }
    
    /// Returns the mode for each `Unit`
    pub fn modes(&self) -> Stat { self.collected_stats.modes() }
    
    /// Returns the standard deviation for each `Unit`
    pub fn std_deviations(&self) -> Stat { self.collected_stats.std_deviations() }
}
impl Roll for StatisticsRoll {
    /// Simpy returns "Result of # rolls"
    fn intermediate_results(&self) -> String { 
        format!("Result of {} rolls", self.rolls.len()) }
    
    fn final_result(&self) -> String {
        format!("{}:\n{}\n{}\n{}\n{}", 
                self.intermediate_results(),
                self.averages(),
                self.medians(),
                self.modes(), 
                self.std_deviations() ) }
}


/// A collection of all the stats, grouped by `Unit` in `UnitStats`
pub struct CollectedStats {
  stats: Vec<UnitStats>
}
impl CollectedStats {
    /// Returns the `UnitStats` for the given `Unit`
    pub fn for_unit(&self, unit: Rc<dyn Unit>) -> Option<&UnitStats> {
        for rstats in self.stats.iter() {
            if rstats.has_same_unit(unit.clone()) {
                return Some(rstats) } }
        None }
    
    /// Returns the average for each `Unit`
    pub fn averages(&self) -> Stat {
        Stat { 
            stat_type: StatType::Average,
            values: self.stats.iter()
                .map(|rstats| StatValue {
                   unit: rstats.unit.clone(),
                    value: rstats.average })
                .collect() } }
    
    /// Returns the median for each `Unit`            
    pub fn medians(&self) -> Stat {
        Stat {
            stat_type: StatType::Median,
            values: self.stats.iter()
                .map(|rstats| StatValue {
                   unit: rstats.unit.clone(),
                    value: rstats.median })
                .collect() } }
    
    /// Returns the mode for each `Unit`
    pub fn modes(&self) -> Stat {
        Stat {
            stat_type: StatType::Mode,
            values: self.stats.iter()
                .map(|rstats| StatValue {
                   unit: rstats.unit.clone(),
                    value: rstats.mode })
                .collect() } }
    
    /// Returns the standard deviation for each `Unit`
    pub fn std_deviations(&self) -> Stat {
        Stat {
            stat_type: StatType::StdDeviation,
            values: self.stats.iter()
                .map(|rstats| StatValue {
                   unit: rstats.unit.clone(),
                    value: rstats.std_deviation })
                .collect() } }
}


/// Enum to distinguish between the different kinds of statistics gathered
pub enum StatType {
    Average, Median, Mode, StdDeviation
}
impl Display for StatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            StatType::Average => f.write_str("Average"),
            StatType::Median => f.write_str("Media"),
            StatType::Mode => f.write_str("Mode"),
            StatType::StdDeviation => f.write_str("Standard Deviation")
        }
    }
}


/// `Stat` contains the statistics of a single `StatType` (average, median, etc.) for all of the
/// `Unit`s collected.
pub struct Stat {
    stat_type: StatType,
    values: Vec<StatValue>
}
impl Stat {
    /// Look up the value of this `Stat`'s `StatType` for the given `Unit`, if there is one
    pub fn for_unit(&self, unit: Rc<dyn Unit>) -> Option<f32> {
        self.values.iter()
            .filter(|stat_val| stat_val.has_same_unit(unit.clone()))
            .map(|stat_val| stat_val.value)
            .next() }
    
    /// Iterator to cycle through the stats by `Unit`
    pub fn iter(&self) -> impl Iterator<Item=&StatValue> { self.values.iter() }
}
impl <'a> IntoIterator for &'a Stat {
   type Item = &'a StatValue;
   type IntoIter = std::slice::Iter<'a, StatValue>;
   fn into_iter(self) -> <&'a Stat as IntoIterator>::IntoIter { self.values.iter() }
}
impl Display for Stat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("{}:\n", self.stat_type))?;
        for sv in self.iter(){
            f.write_fmt(format_args!("{}\n", sv))? }
        Ok(()) }
}
// TODO: We probably need to include the StatType inside the returned StatValue...
// Current thought: 
// -  Rename StatValue to UnitValue (stop making it public)
// -  Keep storing UnitValue in Stat
// -  Make a new type called StatValue that stores StatType, Unit, and value
// -  Iterators in Stat map the UnitValues to StatValues


pub struct StatValue {
    unit: Rc<dyn Unit>,
    value: f32,
}
impl StatValue {
    pub fn has_same_unit(&self, unit: Rc<dyn Unit>) -> bool {
        self.unit.deref() == unit.deref() }
}
impl Display for StatValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {:.2}", self.unit, self.value)) }
}


/// Holds all of the stats for a certain `Unit` as well as all of the values that were used to
/// calculate those stats (in no particular order).
pub struct UnitStats {
    pub unit: Rc<dyn Unit>,
    pub values: Vec<i32>,
    pub average: f32,
    pub median: f32,
    pub mode: f32,
    pub std_deviation: f32
}
impl UnitStats {
    pub fn has_same_unit(&self, unit: Rc<dyn Unit>) -> bool {
        self.unit.deref() == unit.deref() }
}
