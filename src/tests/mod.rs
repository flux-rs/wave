mod fuzz_trusted;
mod integration_tests;
mod unit_tests;

//mod test_gen_trace;
#[cfg(test)]
mod test_generator;

//use crate::types::Stat;
//use crate::tcb::verifier::*;

/// Any common initialization for the tests (e.g. changing the working directory)
pub fn init() {
    // this will only actually happen once, but just call it everytime
    // it will fail but that is fine...
    let _ = std::env::set_current_dir("./fuzz-dir");
}
