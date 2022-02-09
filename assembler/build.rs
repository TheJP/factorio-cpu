use std::{env, fs::File, io::{self, Write}, path::PathBuf};

use glob::glob;

/// Generate tests for files in ./tests/data.
fn main() -> io::Result<()> {
    // Addind/Removing files in ./tests/data should trigger a rebuild.
    println!("cargo:rerun-if-changed=tests/data");

    let mut out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_path.push("generated_tests.trs");
    // println!("gen: {}", out_path.display());
    let mut out_file = File::create(out_path)?;

    let mut i = 1;

    for asm_file in glob("tests/data/**/*.asm").expect("Error while searching test files") {
        let asm_file = asm_file.unwrap();
        let bin_file = asm_file.with_extension("bin");
        assert!(bin_file.exists());

        writeln!(
            out_file,
            include_str!("tests/templates/assemble_test.trs"),
            number = i,
            asm_path = asm_file.display(),
            bin_path = bin_file.display(),
        )?;

        i += 1;
    }

    Ok(())
}
