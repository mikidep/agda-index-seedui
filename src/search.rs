use probly_search::{
    index::{add_document_to_index, create_index, Index},
    query::{query, score::default::bm25, QueryResult},
};

use crate::model::Reference;

pub struct SearchEngine {
    pub haystack: Vec<Reference>,
    pub index: Index<usize>,
}

fn tokenizer(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}

fn identifier_accessor(refr: &Reference) -> Option<&str> {
    Some(refr.identifier.as_str())
}

fn id_filter(s: &str) -> String {
    s.to_string()
}

impl SearchEngine {
    pub fn new(haystack: Vec<Reference>) -> Self {
        let mut index = create_index(1);
        for (idx, refr) in haystack.clone().into_iter().enumerate() {
            add_document_to_index(
                &mut index,
                &[identifier_accessor],
                tokenizer,
                id_filter,
                idx,
                refr,
            );
        }
        Self { haystack, index }
    }

    pub fn search(&mut self, needle: &str) -> Vec<Reference> {
        query(
            &mut self.index,
            needle,
            &mut bm25::new(),
            tokenizer,
            id_filter,
            &[1.],
            None,
        )
        .into_iter()
        .filter_map(|qr| self.haystack.get(qr.key))
        .cloned()
        .collect()
    }
}
