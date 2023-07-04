use crate::model::{CountedReference, Reference};

pub trait SearchEngine {
    fn new(haystack: Vec<Reference>) -> Self;
    fn search(&mut self, needle: &str) -> Vec<CountedReference>;
    fn all(&self) -> Vec<CountedReference>;
}

pub mod probly;
pub mod fuzzy;