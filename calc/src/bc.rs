//
// Copyright (c) 2024 Hemi Labs, Inc.
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

use std::ffi::OsString;

use bc_util::{
    interpreter::Interpreter,
    parser::{is_incomplete, parse_program},
};
use clap::Parser;

use gettextrs::{bind_textdomain_codeset, textdomain};
use plib::PROJECT_NAME;
use rustyline::{error::ReadlineError, DefaultEditor, Result};

mod bc_util;

/// bc - arbitrary-precision arithmetic language
#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short = 'l')]
    define_math_functions: bool,

    files: Vec<OsString>,
}

fn exec_str(s: &str, file_path: Option<&str>, interpreter: &mut Interpreter) {
    match parse_program(s, file_path) {
        Ok(program) => match interpreter.exec(program) {
            Ok(output) => {
                print!("{}", output);
            }
            Err(e) => {
                println!("runtime error: {}", e);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main() -> Result<()> {
    textdomain(PROJECT_NAME)?;
    bind_textdomain_codeset(PROJECT_NAME, "UTF-8")?;

    let args = Args::parse();
    let mut interpreter = Interpreter::default();

    if args.define_math_functions {
        let lib = parse_program(include_str!("bc_util/math_functions.bc"), None)
            .expect("error parsing standard math functions");
        interpreter
            .exec(lib)
            .expect("error loading standard math functions");
    }

    for file in args.files {
        match std::fs::read_to_string(&file) {
            Ok(s) => exec_str(&s, file.to_str(), &mut interpreter),
            Err(_) => {
                eprintln!("Could not read file: {}", file.to_string_lossy());
                return Ok(());
            }
        };
        if interpreter.has_quit() {
            return Ok(());
        }
    }

    let mut repl = DefaultEditor::new()?;
    let mut line_buffer = String::new();
    while !interpreter.has_quit() {
        let line = if line_buffer.is_empty() {
            repl.readline(">> ")
        } else {
            repl.readline(".. ")
        };
        match line {
            Ok(line) => {
                line_buffer.push_str(&line);
                line_buffer.push('\n');
                if !is_incomplete(&line_buffer) {
                    exec_str(&line_buffer, None, &mut interpreter);
                    line_buffer.clear();
                }
                repl.add_history_entry(line)?;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
