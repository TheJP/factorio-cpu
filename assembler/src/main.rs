mod ir;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{App, Arg};

use crate::ir::{TranslationTable, IR};

fn parse_arguments() -> Option<String> {
    let matches = App::new("JP Factorio Assembler")
        .version("0.1.0")
        .arg(
            Arg::new("input-file")
                .help("Assembly file that is going to be assembled")
                .required(true),
        )
        .get_matches();

    match matches.value_of("input-file") {
        Some(file) => Some(file.into()),
        _ => None,
    }
}

fn main() {
    let file_name = if let Some(file) = parse_arguments() {
        file
    } else {
        eprintln!("Invalid argument(s). Try --help for more information.");
        return;
    };

    let translation = TranslationTable::new();

    let file = File::open(file_name).expect("Could not open file");
    let intermediate = IR {
        instructions: BufReader::new(file)
            .lines()
            .enumerate()
            .filter_map(|(number, line)| translation.create_intermediate(&line.expect("Error while reading from file."), number))
            .collect(),
    };

    println!("{:#?}", intermediate);
}
