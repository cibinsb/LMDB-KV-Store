use std::collections::HashMap;
use fst::Set;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use charabia::{Tokenizer, TokenizerBuilder};
use crate::datastore::{Docs, KV, LMDBStore};
use serde::{Deserialize};
use lazy_static::lazy_static;

lazy_static! {
    static ref STOP_WORDS: Set<Vec<u8>> = Set::from_iter(
        ["a", "an", "and", "as", "at", "be", "but",
        "by", "for", "from", "has", "he", "in", "is","it", "its", "of", "on", "that", "the",
        "to", "was", "were", "will", "with"].iter()
    ).unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub(crate) query: String,
}

pub struct PreProcessingPipeline {
    pub(crate) tokenizer: Tokenizer<'static, 'static, Vec<u8>>,
}

impl PreProcessingPipeline {
    pub(crate) fn new() -> Self {
        let mut builder = TokenizerBuilder::new();
        let builder = builder.stop_words(&*STOP_WORDS);
        Self { tokenizer: builder.build() }
    }
}

pub fn preprocess(text: &str, tokenizer: &Tokenizer<Vec<u8>>) -> Vec<String> {
    let tokens: Vec<_> = tokenizer.tokenize(text).collect();
    tokens
        .iter()
        .filter(|t|t.is_word())
        .map(|t| t.lemma().to_string())
        .collect()
}

pub fn index(
    kv: KV,
    datastore: &RwLockWriteGuard<LMDBStore>,
    key: String,
) {
    let tokenizer = &datastore.preprocessor_pipeline.tokenizer;
    let tokens = preprocess(&kv.log, tokenizer);
    let rtxn = datastore.env.read_txn().unwrap();
    let mut wtxn = datastore.env.write_txn().unwrap();
    for token in tokens {
        let mut keys =
            match datastore.inverted_index.get(&rtxn, &token).unwrap() {
            Some(docs) => docs.keys,
            _ => Vec::new(),
        };
        keys.push(key.clone());
        let docs = Docs {
            keys
        };
        datastore
        .inverted_index
        .put(&mut wtxn, &token, &docs)
        .unwrap();
    }
    wtxn.commit().unwrap();
}

pub fn value_search(query: String, datastore: &RwLockReadGuard<LMDBStore>) -> HashMap<String, String> {
    let rtxn = datastore.env.read_txn().unwrap();
    let keys = match datastore.inverted_index.get(&rtxn, &query).unwrap() {
            Some(docs) => docs.keys,
            _ => Vec::new(),
    };
    let mut resultset: HashMap<String, String> = Default::default();
    for key in keys {
        let kv = datastore.db.get(&rtxn, &key).unwrap();
        resultset.insert(key, kv.unwrap().log);
    }
    resultset
}