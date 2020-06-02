#[macro_use]
extern crate serde_derive;

//use std::io;
mod process_bpmn;

fn main() {
    let _test=process_bpmn::Process::ProcessId(3);
    println!("Hello, world!");
}
