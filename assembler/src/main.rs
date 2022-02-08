use std::{
    fs::{self},
};

use clap::{App, Arg};

use lib::assemble;

const DEFAULT_OUTPUT: &str = "out.bin";

struct Arguments {
    input_file: String,
    output_file: String,
}

fn parse_arguments() -> Option<Arguments> {
    let matches = App::new("JP Factorio Assembler")
        .version("0.1.0")
        .arg(
            Arg::new("input-file")
                .help("Assembly file that is going to be assembled")
                .required(true),
        )
        .arg(
            Arg::new("output-file")
                .short('o')
                .value_name("FILE")
                .default_value(DEFAULT_OUTPUT)
                .help("Output file to which the assembled binary output is written"),
        )
        .get_matches();

    match (matches.value_of("input-file"), matches.value_of("output-file")) {
        (Some(input_file), output_file) => Some(Arguments {
            input_file: input_file.into(),
            output_file: output_file.unwrap_or(DEFAULT_OUTPUT).into(),
        }),
        _ => None,
    }
}

fn main() {
    let args = if let Some(file) = parse_arguments() {
        file
    } else {
        eprintln!("Invalid argument(s). Try --help for more information.");
        return;
    };

    let assembled = assemble(&args.input_file);
    fs::write(args.output_file, &assembled).expect("Could not create output file");
}
