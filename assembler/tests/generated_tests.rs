mod common;

// Include tests generated by `build.rs`.
include!(concat!(env!("OUT_DIR"), "/generated_tests.trs"));