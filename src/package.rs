use fs::metadata as stat;
use fs_err as fs;
use std::{collections::HashMap, error};

use serde_derive::Deserialize;

// only relevant parts that we care about
#[derive(Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub scripts: Option<HashMap<String, String>>,
}

pub fn parse() -> Result<PackageJson, Box<dyn error::Error>> {
    let pnpm_dne = || stat("pnpm-lock.yaml").is_err();
    let yarn_dne = || stat("yarn.lock").is_err();
    let npm_dne = || stat("package-lock.json").is_err();

    if pnpm_dne() && yarn_dne() && npm_dne() {
        return Err("could not assert existence of pnpm, npm, or yarn lockfiles, rpnx is only designed for pnpm,npm,yarn(classic) projects".into());
    }

    let pjson = serde_json::from_reader(fs::File::open("package.json")?)
        .map_err(|e| format!("reading package.json: {e}"))?;

    Ok(pjson)
}
