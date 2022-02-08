mod common;

#[test]
fn example() {
    common::assemble_test("tests/data/mov.asm", "tests/data/mov.bin");
}
