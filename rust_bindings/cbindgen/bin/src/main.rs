extern crate cbindgen;
extern crate clap;
extern crate log;

use clap::{App, Arg};

fn matcher() -> App<'static, 'static> {
	App::new("cbindgen_bazel")
        .about("Generate C bindings for a Rust library")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Enable verbose logging")
        )
        .arg(
            Arg::with_name("config")
                .help("Specify path to a `cbindgen.toml` config to use")
                .required(true)
                .multiple(false)
                .takes_value(true)
                .value_name("CONFIGFILE")
        )
        .arg(
            Arg::with_name("out")
                .long("output")
                .help("The file to output the bindings to")
                .required(true)
                .value_name("FILE")
        )
        .arg(
            Arg::with_name("files")
                .long("files")
                .help("A list of source files")
                .required(true)
                .multiple(true)
                .value_name("FILES")
        )
}

fn main() {
	let m = matcher().get_matches();
	let files: Vec<&str> = m.values_of("files").unwrap().collect();
	let out = m.value_of("out").unwrap();
	let config = cbindgen::Config::from_file(m.value_of("config").unwrap()).unwrap();

    // Initialize logging
    match m.occurrences_of("v") {
        0 => logging::WarnLogger::init().unwrap(),
        1 => logging::InfoLogger::init().unwrap(),
        _ => logging::TraceLogger::init().unwrap(),
    }

	let mut builder = cbindgen::Builder::new().with_config(config);
	for f in files {
        builder = builder.with_src(f);
    }
    builder.generate()
    	.expect("Unable to generate bindings")
        .write_to_file(out);



    //cbindgen::Builder::new()
    //  .with_crate(crate_dir)
    //  .generate()
    //  .expect("Unable to generate bindings")
    //  .write_to_file("bindings.h");
}

mod logging {
	/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io;
use std::io::Write;

use log;
use log::*;

pub struct TraceLogger;
pub struct WarnLogger;
pub struct InfoLogger;

impl TraceLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&InfoLogger)?;
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}
impl log::Log for TraceLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            writeln!(io::stderr(), "{}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {
        io::stderr().flush().unwrap();
    }
}

impl WarnLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&InfoLogger)?;
        log::set_max_level(LevelFilter::Warn);
        Ok(())
    }
}
impl log::Log for WarnLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Warn
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            writeln!(io::stderr(), "{}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {
        io::stderr().flush().unwrap();
    }
}

impl InfoLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&InfoLogger)?;
        log::set_max_level(LevelFilter::Info);
        Ok(())
    }
}
impl log::Log for InfoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            writeln!(io::stderr(), "{}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {
        io::stderr().flush().unwrap();
    }
}
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {
		let m = super::matcher().get_matches_from(vec!["--config", "cbindgen.toml", "--files", "file1", "file2", "file3", "--output", "output.h"]);
		let files: Vec<&str> = m.values_of("files").unwrap().collect();
		let out = m.value_of("out").unwrap();
		let config = m.value_of("config").unwrap();
        assert_eq!(config, "cbindgen.toml");
        assert_eq!(files, ["file1", "file2", "file3"]);
        assert_eq!(out, "output.h");
    }
}