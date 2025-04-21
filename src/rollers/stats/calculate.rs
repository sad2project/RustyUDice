use std::{
    collections::HashMap,
    rc::Rc };
use crate::{
    {Relationship, Value, Values},
    rollers::CollectedStats };
use crate::rollers::RelationshipStats;

pub struct CollectedStatsBuilder {
    stats: Vec<RelationshipStatsCalculator>
}
impl CollectedStatsBuilder {
    pub(crate) fn new(roll_vals: Vec<Values>) -> Self { 
        let size = roll_vals.len();
        let mut out = Self { stats: Vec::new() };
        for values in roll_vals {
            out.add_roll_values(values, size as u32);
        }
        out.finish();
        out
    }

    fn add_roll_values(&mut self, values: Values, size: u32) {
        for value in values.into_iter() {
            self.add_value(value.clone(), size) }
    }

    fn add_value(&mut self, value: Value, size: u32) {
        for rvalues in self.stats.iter_mut() {
            // attempts to add the Value and returns on success
            if rvalues.add_on_match(value.clone()) { 
                return; } }
        // if we get here, there was no match, so we make a new one and add it
        self.stats.push(
            RelationshipStatsCalculator::new(value, size));
    }

    fn finish(&mut self) {
        for rvalues in self.stats.iter_mut() {
            rvalues.finish(); }
    }

    pub(crate) fn build(self) -> CollectedStats {
        CollectedStats { 
            stats: self.stats.into_iter()
                .map(|rvalues| rvalues.calc_all())
                .collect() } }
}


pub struct RelationshipStatsCalculator {
    relationship: Rc<dyn Relationship>,
    values: Vec<i32>
}

impl RelationshipStatsCalculator {
    fn new(value: Value, size: u32) -> Self {
        let mut values = Vec::with_capacity(size as usize);
        values.push(value.value);
        Self { relationship: value.relationship, values }
    }

    fn add_on_match(&mut self, value: Value) -> bool {
        if self.has_same_relationship(value.relationship) {
            self.values.push(value.value);
            true }
        else {
            false } }

    pub fn has_same_relationship(&self, relationship: Rc<dyn Relationship>) -> bool {
        self.relationship.id() == relationship.id() }
    
    // Fill the rest of the space with 0s and sort the values    
    fn finish(&mut self) {
        let num_needed = self.values.capacity() - self.values.len();
        for _ in 0..num_needed {
            self.values.push(0); }
        self.values.sort();
    }

    pub fn average(&self) -> f32 {
        let sum = self.values.iter().sum::<i32>() as f32;
        sum / (self.values.len() as f32) 
    }

    pub fn median(&self) -> f32 {
        let mid = self.values.len() / 2;
        if self.values.len().is_multiple_of(2) {
            (self.values[mid - 1] + self.values[mid]) as f32 / 2.0 }
        else {
            self.values[mid] as f32 }
    }

    /// In the case that there are multiple mode values, an indeterminate "first"
    /// value will be returned
    pub fn mode(&self) -> f32 {
        let mut map = HashMap::new();
        for value in self.values.iter() {
            let count = map.entry(value).or_insert(0);
            *count += 1;
        }

        let max_value = map.values().cloned().max().unwrap_or(0);

        *map.into_iter()
            .filter(|&(_, v)| v == max_value)
            .map(|(k, _)| k)
            .nth(0).unwrap() as f32
    }

    /// Asks for the average because it has most likely been calculated already,
    /// and it would be silly to recalculate it and questionable to cache the 
    /// calculation internally
    pub fn std_deviation(&self, avg: f32) -> f32 {
        self.values.iter()
            // collect distance from the average squared
            .map(|val| (avg - (*val as f32)).powi(2))
            // add them up and divide by the length - this gives the variance
            .sum::<f32>() / (self.values.len() as f32)
            // square root to finish with standard deviation
            .sqrt() }

    pub fn calc_all(self) -> RelationshipStats {
        let average = self.average();
        RelationshipStats {
            average,
            median: self.median(),
            mode: self.mode(),
            std_deviation: self.std_deviation(average),
            relationship: self.relationship.clone(),
            values: self.values, }
    }
}
