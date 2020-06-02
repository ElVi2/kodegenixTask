use serde::{Serialize, Deserialize};
use std::io;
mod process_bpmn;

fn main() {
    let mut test=process_bpmn::Process::ProcessId(3);
    println!("Hello, world!");
}
