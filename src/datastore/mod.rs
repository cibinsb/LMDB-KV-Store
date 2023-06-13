use heed::types::*;
use heed::{Database, Env, EnvOpenOptions};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Nextflow {
    pub log: String,
}
pub struct LMDBStore {
    pub env: Env,
    pub db: Database<Str, SerdeBincode<Nextflow>>,
}

impl Default for LMDBStore {
    fn default() -> Self {
        fs::create_dir_all(Path::new("data").join("bytemuck.mdb")).unwrap();
        let env = EnvOpenOptions::new()
            .map_size(10000000 * 1024 * 1024)
            .open(Path::new("data").join("bytemuck.mdb"))
            .unwrap();
        // we will open the default unamed database
        let db: Database<Str, SerdeBincode<Nextflow>> = env.create_database(None).unwrap();
        Self { env, db }
    }
}
