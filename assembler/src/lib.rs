mod ir;
mod assembler;

use std::{fs::File, io::{BufReader, BufRead}, path::Path};

use crate::{ir::{IR, IRTranslationTable}};

pub fn assemble<P: AsRef<Path>>(input_file: P) -> Vec<u8> {
    let translation = IRTranslationTable::new();

    let file = File::open(input_file).expect("Could not open input file");
    let intermediate = IR {
        instructions: BufReader::new(file)
            .lines()
            .enumerate()
            .filter_map(|(number, line)| translation.create_intermediate(&line.expect("Error while reading from file."), number + 1))
            .collect(),
    };

    return assembler::assemble(intermediate);
}