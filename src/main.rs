#![feature(try_blocks)]
use std::io::Read;
use std::fs::File;
mod process_bpmn;
mod helper;
mod bpmn_parse;

fn main() {
    //let _test=process_bpmn::FlowObject::Activity(process_bpmn::Activity::Task);
    println!("Please input the path:");
    //let mut file_path = String::new();
    //io::stdin()
        //.read_line(&mut file_path)
        //.expect("Failed to read line!");
    let file_path="data/Call Activity.bpmn";
    println!("{}", file_path);
    let mut bpmn_file = File::open(file_path).unwrap();
    let mut contents = String::new();
    bpmn_file.read_to_string(&mut contents).unwrap();
    let def = bpmn_parse::parse_process(contents);
    //println!("Parsed file: {:?}", def);
    for process in def.processes {
        println!("Process {}:", process.id);
        for node in process.nodes { println!("{:?}", node); }
        for subprocess in process.subprocesses { println!("{:?}", subprocess); }
    }
}
