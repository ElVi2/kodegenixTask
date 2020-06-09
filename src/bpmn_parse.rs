use super::*;
use quick_xml;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use process_bpmn;
use helper;
use process_bpmn::{Definitions, SubProcess, Process, Node, FlowObject};
use helper::{parse_attributes, get_id, get_name};



pub fn parse_process(contents: String)->process_bpmn::Definitions {
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut def=Definitions{id:" ".to_string(),processes:Vec::new()};
    let mut proc=Process{is_executable: false, id:"default".to_string(), nodes: Vec::new(), subprocesses: Vec::new()};
    let mut node = Node {id: "default".to_string(), name: "default".to_string(), flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
    let mut text_switch = 0; //this will indicate which tag was the last if there is plain text inside the tag
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"semantic:definitions" => {let definition_attributes = parse_attributes(e);
                        def.id=definition_attributes[0].clone();
                    },
                    b"semantic:process"=>{let process_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        proc.nodes.remove(0);
                        node=Node {id: "default".to_string(), name: "default".to_string(), flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
                        def.processes.push(proc);
                        println!("attributes values: {:?}", &process_attributes);
                        proc=Process{is_executable: process_attributes[0].parse::<bool>().unwrap(),
                            id: process_attributes[1].clone(), nodes: Vec::new(), subprocesses: Vec::new()};
                    },
                    b"semantic:incoming"=>{
                        text_switch=1;  //set the switch to recognize the incoming connection tag
                        println!("{ }",text_switch);
                    },
                    b"semantic:outgoing"=>{
                        text_switch=2;  //set the switch to recognize the outgoing connection tag
                        println!("{ }",text_switch);
                    },
                    b"semantic:sequenceFlow"=> {

                    },
                    b"semantic:subProcess" => {
                        println!("Found subprocess!");
                        let subprocess_attributes = parse_attributes(e);
                        println!("attributes values: {:?}", &subprocess_attributes);
                        parse_subprocess(&mut reader, &mut proc.subprocesses, &mut buf, subprocess_attributes);
                    },
                    /*
                    b"bpmndi:BPMNDiagram"=>println!("Ok!"),
                    b"bpmndi:BPMNPlane"=>println!("Ok!"),
                    b"bpmndi:BPMNShape"=>println!("Ok!"),
                    b"bpmndi:BPMNLabel/"=>println!("Ok!"),
                    b"dc:Bounds"=>println!("Ok!"),
                    b"bpmndi:BPMNEdge"=>println!("Ok!"),
                    b"di:waypoint"=>println!("Ok!"),
                    b"bpmndi:BPMNDiagram"=>println!("Ok!"),*/
                    //commented code for future usage
                    //_ => println!("{}", String::from_utf8_lossy(e.name())),
                    //_ => println!(),
                    _ => {
                        let node_attributes = parse_attributes(e);
                        if let Ok(n) = parse_node(e) {
                            proc.nodes.push(node);
                            node = n;
                            println!("attributes values: {:?}", node_attributes);
                        }
                    },
                }
            },
            Ok(Event::Text(e)) => {
                let text_var=e.unescape_and_decode(&reader).unwrap();
                match text_switch { // how do we handle the text inside the tags
                    1 => process_bpmn::add_connection(&mut node, text_switch, text_var.clone()),
                    2 => process_bpmn::add_connection(&mut node, text_switch, text_var.clone()),
                    _ =>  println!(),
                }
                println!("{:?}", &text_var);
            },
            Ok(Event::Eof) => {
                proc.nodes.remove(0);
                proc.nodes.push(node);
                def.processes.push(proc);
                def.processes.remove(0);
                break;
            }, // exits the loop when reaching end of file

            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    def
}

pub fn parse_subprocess(reader: &mut Reader<&[u8]>, subprocesses: &mut Vec<SubProcess>, buf: &mut Vec<u8>, attr: Vec<String>) {
    let mut subproc=SubProcess{triggered_by_event: attr[0].clone().parse::<bool>().unwrap(),
        start_quantity: attr[3].clone().parse::<u64>().unwrap(),
        completion_quantity: attr[1].clone().parse::<u64>().unwrap(),
        up_for_compensation: attr[2].clone().parse::<bool>().unwrap(),
        id: attr[4].clone(), name:attr[5].clone(), nodes: Vec::new(), subprocesses: Vec::new()};
    let mut node = Node {id: "default".to_string(), name: "default".to_string(),
        flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
    let mut text_switch = 0;
    loop {
        println!("Cycle!");
        match reader.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    _ => ()
                }
            }
            Ok(Event::Text(e)) => {
                let text_var=e.unescape_and_decode(&reader).unwrap();
                println!("{:?}", &text_var);
            },
            Ok(Event::End(ref e)) => {
                if e.name() == b"semantic:subProcess" {
                    //subproc.nodes.remove(0);
                    subproc.nodes.push(node);
                    subprocesses.push(subproc);
                    break;
                }
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }

    }
}

pub fn parse_node(e: &BytesStart) -> Result<Node, String> {
    let mut node;
    //=Node {id: "default".to_string(), name: "default".to_string(), flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
    match e.name() {
        b"semantic:startEvent"=>{let node_attributes = parse_attributes(e);
            node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                flow_object: FlowObject::Event(process_bpmn::Event::StartEvent), connections: Vec::new()};
        },
        b"semantic:exclusiveGateway"=>{let node_attributes = parse_attributes(e);
            node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                flow_object: FlowObject::Gateway(process_bpmn::Gateway::ExclusiveGateway), connections: Vec::new()};
        },
        b"semantic:parallelGateway"=>{let node_attributes = parse_attributes(e);
            node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                flow_object: FlowObject::Gateway(process_bpmn::Gateway::ParallelGateway), connections: Vec::new()};
        },
        b"semantic:task"=>{let node_attributes = parse_attributes(e);
            node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                flow_object: FlowObject::Activity(process_bpmn::Activity::Task), connections: Vec::new()};
        },
        b"semantic:userTask"=>{let node_attributes = parse_attributes(e);
            node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                flow_object: FlowObject::Activity(process_bpmn::Activity::UserTask), connections: Vec::new()};
        },
        b"semantic:endEvent"=>{let node_attributes = parse_attributes(e);
            node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                flow_object: FlowObject::Event(process_bpmn::Event::EndEvent), connections: Vec::new()};
        },
        b"semantic:callActivity"=>{let node_attributes = parse_attributes(e);
            node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                flow_object: FlowObject::Activity(process_bpmn::Activity::CallActivity), connections: Vec::new()};
        },
        _ => return Err("Invalid node".into()),
    }
    Ok(node)
}