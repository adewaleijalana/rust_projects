#![allow(dead_code, unused_variables, unused_imports)]

use grep_lite::data_types::{bit_shifting, int_overflow_example, int_vs_int, transmute_test};
use grep_lite::file::{File, close, file_demo, open};
use grep_lite::read::Read;
use grep_lite::reader_args;
use grep_lite::sat::sat_test;

fn main() {
    // reader_args()
    // file_test();
    // sat_test()
    // int_vs_int();
    transmute_test();
    // int_overflow_example();
    // bit_shifting();
}
