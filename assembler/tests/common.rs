use std::{
    cmp, fs,
    path::PathBuf,
};

use colored::Colorize;
use lib::assemble;

fn source_path(path: &str) -> PathBuf {
    let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push(path);
    buf
}

pub fn assemble_test(asm_file: &str, expected_output_file: &str) {
    let actual = assemble(source_path(asm_file));
    let expected = fs::read(source_path(expected_output_file)).unwrap();

    if expected != actual {
        println!("Assemble Test Failed");
        println!("Source: {}", asm_file);
        println!("Expected Output: {}", expected_output_file);
        print_difference(&expected, &actual);

        assert_eq!(expected, actual);
    }
}

fn print_difference(expected: &[u8], actual: &[u8]) {
    const BYTES_PER_ROW: usize = 8;
    const COLUMN_WIDTH: usize = 3 * BYTES_PER_ROW;
    const TABLE_WIDTH: usize = 2 * COLUMN_WIDTH + 1;
    println!("{}", "-".repeat(TABLE_WIDTH));
    println!(
        "{:^width$}|{:^width$}",
        "actual",
        "expected",
        width = COLUMN_WIDTH,
    );
    println!("{}", "-".repeat(TABLE_WIDTH));

    let max = cmp::max(expected.len(), actual.len());
    for i in (0..max).step_by(BYTES_PER_ROW) {
        for j in i..i + BYTES_PER_ROW {
            if j < actual.len() {
                if j < expected.len() && expected[j] == actual[j] {
                    print!("{:02x} ", actual[j])
                } else {
                    print!("{} ", format!("{:02x}", actual[j]).red());
                }
            } else {
                print!("   ")
            }
        }

        print!("|");

        for j in i..i + BYTES_PER_ROW {
            if j < expected.len() {
                if j < actual.len() && expected[j] == actual[j] {
                    print!(" {:02x}", expected[j])
                } else {
                    print!(" {}", format!("{:02x}", expected[j]).green());
                }
            } else {
                print!("   ")
            }
        }

        println!();
    }

    println!("{}", "-".repeat(TABLE_WIDTH));
}
