use fs_err as fs;
use std::{collections::HashMap, error, io};

use serde_derive::Deserialize;

// only relevant parts that we care about
#[derive(Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub scripts: HashMap<String, String>,
}

pub fn parse() -> Result<PackageJson, Box<dyn error::Error>> {
    
    let pnpm = "pnpm-lock.yaml";
    let npm = "package-lock.json";

    if fs::metadata(pnpm).is_err_and(|_| fs::metadata(npm).is_err()) {
        return Err(format!("could not assert existence of {pnpm} or {npm}, rpnx is only designed for pnpm or npm projects").into());
    }

    let pjson = serde_json::from_reader(fs::File::open("package.json")?)?;

    Ok(pjson)
}
