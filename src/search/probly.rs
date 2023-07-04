use probly_search::{
    index::{add_document_to_index, create_index, Index},
    query::{query, score::default::bm25},
};

use crate::model::{CountedReference, Reference};

use super::SearchEngine;


pub struct ProblySearchEngine {
    pub haystack: Vec<CountedReference>,
    pub index: Index<usize>,
}

fn tokenizer(s: &str) -> Vec<String> {
    s.split(&[' ', '.', '\n']).map(|s| s.to_string()).collect()
}

fn identifier_accessor<'a>(refr: &'a &Reference) -> Option<&'a str> {
    Some(refr.identifier.as_str())
}

fn qualified_accessor<'a>(refr: &'a &Reference) -> Option<&'a str> {
    Some(refr.qualified.as_str())
}

fn content_accessor<'a>(refr: &'a &Reference) -> Option<&'a str> {
    Some(refr.content.as_str())
}

fn lowercase_filter(s: &str) -> String {
    s.to_string().to_lowercase()
}

impl SearchEngine for ProblySearchEngine {
    fn new(haystack: Vec<Reference>) -> Self {
        let mut index = create_index(3);
        let counted_haystack: Vec<CountedReference> =
            haystack.into_iter().enumerate().collect();
        for (idx, refr) in counted_haystack.iter() {
            add_document_to_index(
                &mut index,
                &[
                    |r| identifier_accessor(r),
                    |r| qualified_accessor(r),
                    |r| content_accessor(r),
                ],
                tokenizer,
                lowercase_filter,
                *idx,
                refr,
            );
        }
        Self {
            haystack: counted_haystack,
            index,
        }
    }

    fn search(&mut self, needle: &str) -> Vec<CountedReference> {
        query(
            &mut self.index,
            needle,
            &mut bm25::new(),
            tokenizer,
            lowercase_filter,
            &[1., 0.75, 0.25],
            None,
        )
        .into_iter()
        .filter_map(|qr| self.haystack.get(qr.key))
        .cloned()
        .collect()
    }

    fn all(&self) -> Vec<CountedReference> {
        self.haystack.clone()
    }
}
