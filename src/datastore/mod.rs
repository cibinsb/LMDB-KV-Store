use std::env;
use heed::types::*;
use heed::{Database, Env, EnvOpenOptions};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct KV {
    pub log: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Docs {
    pub keys: Vec<String>,
}

pub struct LMDBStore {
    pub env: Env,
    pub db: Database<Str, SerdeBincode<KV>>,
    pub inverted_index: Database<Str, SerdeBincode<Docs>>,
}

impl Default for LMDBStore {
    fn default() -> Self {
        let data_base_name = env::var("DATABASE_NAME").expect("Missing DATABASE_NAME!");
        fs::create_dir_all(Path::new("data").join(&data_base_name)).unwrap();
        let env = EnvOpenOptions::new()
            .map_size(10 * 1024 * 1024 * 1024)
            .max_dbs(2)
            .open(Path::new("data").join(data_base_name))
            .unwrap();
        // we will open the default unnamed database
        let db: Database<Str, SerdeBincode<KV>> = env.create_database(Some("KV")).unwrap();
        let inverted_index: Database<Str, SerdeBincode<Docs>> = env.create_database(Some("Search")).unwrap();
        Self { env, db, inverted_index }
    }
}
