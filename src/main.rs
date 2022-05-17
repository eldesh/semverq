use std::io::{self, BufRead};

use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde_json::{self as json, json};

fn version_core(version: &semver::Version) -> String {
    format!("{}.{}.{}", version.major, version.minor, version.patch)
}

fn process_query(version: &semver::Version, query: &str) -> String {
    query
        .replace(".major", &format!("{}", version.major))
        .replace(".minor", &format!("{}", version.minor))
        .replace(".patch", &format!("{}", version.patch))
        .replace(".version-core", &format!("{}", version_core(&version)))
        .replace(".pre-release", &format!("{}", version.pre))
        .replace(".pre", &format!("{}", version.pre))
        .replace(".build", &format!("{}", version.build))
}

fn main() -> Result<()> {
    let m = Command::new("semverq")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A cli utility for validating semver, accessing semver structure and converting to json.")
        .arg(
            Arg::new("input")
                .short('i')
                .takes_value(true)
                .help("version string"),
        )
        .arg(
            Arg::new("to-json")
                .short('j')
                .help("convert the input string to a json"),
        )
        .arg(
            Arg::new("query")
                .short('q')
                .takes_value(true)
                .conflicts_with("to-json")
                .help("expand accessors"),
        )
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
                "minor": version.minor,
                "patch": version.patch,
                "pre-release": version.pre.as_str(),
                "build": version.build.as_str()
            }))
            .unwrap()
        );
    } else if let Some(query) = m.value_of("query") {
        println!("{}", process_query(&version, &query));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query() {
        let query = r#"{ "pre-release": .pre-release, "build": .build }"#;
        let input = semver::Version::parse("1.2.3-beta+36a1d2f").unwrap();
        assert_eq!(
            &process_query(input, query),
            r#"{ "pre-release": "beta", "build": "36a1d2f" }"#
        )
    }
}
