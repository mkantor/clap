#![cfg(not(tarpaulin))]

use std::ffi::OsStr;
use std::fs;
use std::process::{Command, Output};

fn run_example<S: AsRef<str>>(name: S, args: &[&str]) -> Output {
    let mut all_args = vec![
        "run",
        "--quiet",
        "--example",
        name.as_ref(),
        "--features",
        "yaml unstable",
        "--",
    ];
    all_args.extend_from_slice(args);

    Command::new(env!("CARGO"))
        .args(all_args)
        .output()
        .expect("failed to run example")
}

fn examples() -> impl Iterator<Item = String> {
    fs::read_dir("examples")
        .expect("couldn't read examples directory")
        .map(|result| result.expect("couldn't get directory entry").path())
        .filter(|path| path.is_file() && path.extension().and_then(OsStr::to_str) == Some("rs"))
        .flat_map(|path| path.file_stem().and_then(OsStr::to_str).map(String::from))
}

#[test]
fn long_help() {
    for example in examples() {
        let output = run_example(&example, &["--help"]);
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
}

#[test]
fn short_help() {
    for example in examples() {
        let output = run_example(&example, &["-h"]);
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
}

#[test]
fn long_version() {
    for example in examples() {
        let output = run_example(&example, &["--version"]);
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
}

#[test]
fn short_version() {
    for example in examples() {
        let output = run_example(&example, &["-V"]);
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
}

#[test]
fn invalid_arguments() {
    for example in examples() {
        let output = run_example(
            &example,
            &[
                "these",
                "arguments",
                "are",
                "not",
                "valid",
                "for",
                "any",
                "example",
            ],
        );
        assert!(!output.status.success());
        assert!(!output.stderr.is_empty());
    }
}
