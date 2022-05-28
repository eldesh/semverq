use std::io::{self, BufRead};

use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde_json::{self as json, json};

mod error;
use crate::error::Error;

fn version_core(version: &semver::Version) -> String {
    format!("{}.{}.{}", version.major, version.minor, version.patch)
}

fn process_query(version: &semver::Version, query: &str) -> String {
    query
        .replace(".major", &format!("{}", version.major))
        .replace(".minor", &format!("{}", version.minor))
        .replace(".patch", &format!("{}", version.patch))
        .replace(".version-core", &version_core(version))
        .replace(".pre-release", &format!("{}", version.pre))
        .replace(".pre", &format!("{}", version.pre))
        .replace(".build", &format!("{}", version.build))
}

fn cli(prog: &str) -> Command<'static> {
    Command::new(prog)
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
        .arg(
            Arg::new("match")
                .short('m')
                .takes_value(true)
                .conflicts_with("to-json")
                .conflicts_with("query")
                .help("checking for a match"),
        )
}

fn main() {
    let m = cli("semverq").get_matches();

    let code = match process(m) {
        Ok(()) => 0,
        Err(err) => {
            if let Some(err) = err.downcast_ref::<Error>() {
                use Error::*;
                eprintln!("{}", err);
                match err {
                    InvalidSemver { .. } => 150,
                    InvalidVersionReq { .. } => 151,
                    NotMatchReq { .. } => 152,
                }
            } else {
                1
            }
        }
    };
    std::process::exit(code)
}

fn parse_semver(text: &str) -> Result<semver::Version> {
    Ok(
        semver::Version::parse(text).map_err(|source| Error::InvalidSemver {
            input: text.to_string(),
            source,
        })?,
    )
}

fn parse_semver_req(text: &str) -> Result<semver::VersionReq> {
    Ok(
        semver::VersionReq::parse(text).map_err(|source| Error::InvalidVersionReq {
            input: text.to_string(),
            source,
        })?,
    )
}

fn process(m: clap::ArgMatches) -> Result<()> {
    let version = if let Some(input) = m.value_of("input") {
        parse_semver(input)?
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let bytes = stdin
            .fill_buf()
            .with_context(|| "Failed to get the contents from the inner buffer of the stdin.")?;

        parse_semver(String::from_utf8_lossy(bytes).trim_end())?
    };

    if m.is_present("to-json") {
        println!(
            "{}",
            json::to_string_pretty(&json!({
                "major": version.major,
                "minor": version.minor,
                "patch": version.patch,
                "pre-release": if version.pre.is_empty() { json!(null) } else { json!(version.pre.as_str()) },
                "build": if version.build.is_empty() { json!(null) } else { json!(version.build.as_str()) }
            }))
            .unwrap()
        );
    } else if let Some(query) = m.value_of("query") {
        println!("{}", process_query(&version, query));
    } else if let Some(match_str) = m.value_of("match") {
        let req = parse_semver_req(match_str)?;
        if !req.matches(&version) {
            return Err(Error::NotMatchReq { version, req }.into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query() {
        use serde_json::Value;
        let query = r#"{ "pre-release": ".pre-release", "build": ".build" }"#;
        let input = semver::Version::parse("1.2.3-beta+36a1d2f").unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&process_query(&input, query)).unwrap(),
            serde_json::from_str::<Value>(r#"{ "pre-release": "beta", "build": "36a1d2f" }"#)
                .unwrap()
        )
    }
}
