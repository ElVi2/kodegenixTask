#[macro_use]
//extern crate quick_xml;

//use std::io;
use std::io::Read;
use std::fs::File;
use quick_xml;
//use crate::process_bpmn::Activity;
use quick_xml::Reader;
use quick_xml::events::Event;
//use quick_xml::de::{from_str, DeError};
//use std::mem;
//use std::io::BufReader;
mod process_bpmn;
mod helper;
use process_bpmn::{Definitions, Process, Node, FlowObject };
//use crate::process_bpmn::Connection;

fn main() {
    //let _test=process_bpmn::FlowObject::Activity(process_bpmn::Activity::Task);
    println!("Please input the path:");
    //let mut file_path = String::new();
    //io::stdin()
        //.read_line(&mut file_path)
        //.expect("Failed to read line!");
    let file_path="F:\\Models.Diagrams\\Call Activity.bpmn";
    println!("{}", file_path);
    let mut bpmn_file = File::open(file_path).unwrap();
    let mut contents = String::new();
    bpmn_file.read_to_string(&mut contents).unwrap();
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut def=Definitions{id:" ".to_string(),processes:Vec::new()};
    let mut proc=Process{is_executable: false, id:" ".to_string(), nodes: Vec::new()};
    let mut node=Node {flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"semantic:definitions" => {let definition_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",&definition_attributes);
                        println!("ID: {:?}",&definition_attributes[0]);
                        def.id=definition_attributes[0].clone();
                    },
                    b"semantic:process"=>{let process_attributes = helper::parse_attributes(e);
                        def.processes.push(proc);
                        println!("attributes values: {:?}", &process_attributes);
                        proc=Process{is_executable: process_attributes[0].parse::<bool>().unwrap(), id: process_attributes[1].clone(), nodes: Vec::new()};
                    },
                    b"semantic:startEvent"=>{let node_attributes = helper::parse_attributes(e);
                        proc.nodes.push(node);
                        println!("attributes values: {:?}",&node_attributes);
                        node=Node {flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
                        println!("attributes values: {:?}",&node_attributes);
                    },
                    b"semantic:incoming"=>{let node_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:outgoing"=>{let connection_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",connection_attributes);
                    },
                    b"semantic:exclusiveGateway"=>{let node_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:parallelGateway"=>{let node_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:task"=>{let node_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:userTask"=>{let node_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:endEvent"=>{let node_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:callActivity"=>{let node_attributes = helper::parse_attributes(e);
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:sequenceFlow"=>println!("Ok!"),
                    b"bpmndi:BPMNDiagram"=>println!("Ok!"),
                    b"bpmndi:BPMNPlane"=>println!("Ok!"),
                    b"bpmndi:BPMNShape"=>println!("Ok!"),
                    b"bpmndi:BPMNLabel/"=>println!("Ok!"),
                    b"dc:Bounds"=>println!("Ok!"),
                    b"bpmndi:BPMNEdge"=>println!("Ok!"),
                    b"di:waypoint"=>println!("Ok!"),
                    b"bpmndi:BPMNDiagram"=>println!("Ok!"),
                    _ => println!("{}", String::from_utf8_lossy(e.name())),
               }
                //println!("{}", e.name().)
            },
            //Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => {
                def.processes.push(proc);
                proc=Process{is_executable: false, id:" ".to_string(), nodes: Vec::new()};
                break;
            }, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("Parsed file: {:?}", def);
}
