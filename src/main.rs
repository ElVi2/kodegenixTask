#[macro_use]
extern crate serde_derive;

//use std::io;
use std::io::Read;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_xml;
//use std::mem;
//use std::io::BufReader;
mod process_bpmn;

fn main() {
    let _test=process_bpmn::FlowObject::Activity;
    println!("Please input the path:");
    //let mut file_path = String::new();
    //io::stdin()
        //.read_line(&mut file_path)
        //.expect("Failed to read line!");
    let file_path="F:\\Models.Diagrams\\Call Activity.bpmn";
    println!("{}", file_path);
    let mut bpmn_file = File::open(file_path).unwrap();
    let mut contents = String::new();
    bpmn_file.read_to_string(&mut contents);
    let deserialized: process_bpmn::Process = serde_xml::from_str(&contents).unwrap();
    println!("{}", contents);
}
