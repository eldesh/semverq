use std::io::{self, BufRead};

use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde_json::{self as json, json};

fn main() -> Result<()> {
    let m = Command::new("SemVerq")
        .about("A cli utility for validating semver, accessing semver structure and converting to json.")
        .arg(Arg::new("input").short('i').takes_value(true))
        .arg(Arg::new("to-json").short('j'))
        .arg(Arg::new("query").short('q').takes_value(true))
        .get_matches();

    let version = if let Some(input) = m.value_of("input") {
        semver::Version::parse(&input)?
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let bytes = stdin
            .fill_buf()
            .with_context(|| "Failed to get the contents from the inner buffer of the stdin.")?;
        semver::Version::parse(&String::from_utf8_lossy(bytes).trim_end())?
    };

    if m.is_present("to-json") {
        println!(
            "{}",
            json::to_string_pretty(&json!({
                "major": version.major,
                "minor":version.minor,
                "patch":version.patch,
                "pre-release":version.pre.as_str(),
                "build":version.build.as_str()
            }))
            .unwrap()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
