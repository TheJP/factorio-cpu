mod ir;
mod assembler;

use std::{fs::File, io::{BufReader, BufRead}};

use crate::{ir::{IR, IRTranslationTable}};

pub fn assemble(input_file: &str) -> Vec<u8> {
    let translation = IRTranslationTable::new();

    let file = File::open(input_file).expect("Could not open input file");
    let intermediate = IR {
        instructions: BufReader::new(file)
            .lines()
            .enumerate()
            .filter_map(|(number, line)| translation.create_intermediate(&line.expect("Error while reading from file."), number))
            .collect(),
    };

    return assembler::assemble(intermediate);
}