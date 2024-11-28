fn escape_one(s: &str) -> String {
    format!("{:?}", s)
}

/// performs escaping similar enough to what pnpm does for packagejson scripts (bad but whatever)
pub fn escape_script(script: &str, args: &[String]) -> String {
    let mut out = script.to_owned();

    for s in args {
        out += " ";
        out += &escape_one(s);
    }

    out
}
