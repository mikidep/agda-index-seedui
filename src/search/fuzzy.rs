use std::collections::BTreeMap;

use fuzzy_matcher::clangd;
use fuzzy_matcher::FuzzyMatcher;

use sublime_fuzzy::best_match;

use crate::model::{CountedReference, Reference};

use super::SearchEngine;

const NK: usize = 3;
const LIMIT: usize = 1;

struct Keys<const N: usize>([String; N]);
type Keygen = fn(refr: &Reference) -> String;

fn identifier_keygen(refr: &Reference) -> String {
    refr.identifier.clone()
}

fn qualified_keygen(refr: &Reference) -> String {
    refr.qualified.clone()
}

fn content_keygen(refr: &Reference) -> String {
    refr.content.clone()
}

const KEYGENS: [Keygen; NK] =
    [identifier_keygen, qualified_keygen, content_keygen];

const RELEVANCES: [i64; NK] = [100, 75, 50];

impl Keys<NK> {
    fn from_refr(refr: &Reference) -> Self {
        Keys(KEYGENS.map(|keygen| keygen(refr)))
    }

    fn zip<'a, Other>(
        &'a self,
        others: &'a [Other; NK],
    ) -> [(&'a str, &'a Other); NK] {
        [
            (self.0[0].as_str(), &others[0]),
            (self.0[1].as_str(), &others[1]),
            (self.0[2].as_str(), &others[2]),
        ]
    }
}

struct SearchItem {
    index: usize,
    refr: Reference,
    keys: Keys<NK>,
}

pub trait Matcher: Default {
    fn fuzzy_match(&self, key: &str, needle: &str) -> Option<i64>;
}

pub struct FuzzySearch<M: Matcher> {
    matcher: M,
    haystack: Vec<SearchItem>,
}

#[derive(Default)]
pub struct ClangdMatcher(clangd::ClangdMatcher);

impl Matcher for ClangdMatcher {
    fn fuzzy_match(&self, key: &str, needle: &str) -> Option<i64> {
        self.0.fuzzy_match(key, needle)
    }
}

#[derive(Default)]
pub struct SublimeMatcher;

impl Matcher for SublimeMatcher {
    fn fuzzy_match(&self, key: &str, needle: &str) -> Option<i64> {
        best_match(needle, key).map(|m| m.score() as i64)
    }
}

pub type TheFuzzySearch = FuzzySearch<SublimeMatcher>;

impl<M: Matcher> SearchEngine for FuzzySearch<M> {
    fn new(haystack: Vec<Reference>) -> Self {
        FuzzySearch {
            matcher: M::default(),
            haystack: haystack
                .into_iter()
                .enumerate()
                .map(|(idx, refr)| SearchItem {
                    index: idx,
                    keys: Keys::from_refr(&refr),
                    refr,
                })
                .collect(),
        }
    }

    fn search(&mut self, needle: &str) -> Vec<CountedReference> {
        let btree = self
            .haystack
            .iter()
            .map(|item| {
                item.keys
                    .zip(&RELEVANCES)
                    .iter()
                    .take(LIMIT)
                    .filter_map(|(key, rel)| {
                        self.matcher.fuzzy_match(key, needle).map(|score| {
                            (-score * *rel, (item.index, item.refr.clone()))
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<BTreeMap<_, _>>();
        btree.into_values().collect()
    }

    fn all(&self) -> Vec<CountedReference> {
        self.haystack
            .iter()
            .map(|item| (item.index, item.refr.clone()))
            .collect()
    }
}
