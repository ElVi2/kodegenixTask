use super::*;
use quick_xml;
use quick_xml::Reader;
use quick_xml::events::Event;
use process_bpmn;
use helper;
use process_bpmn::{Definitions, Process, SubProcess, Node, FlowObject};
use helper::{parse_attributes, get_id, get_name};



pub fn parse_process(contents: String)->process_bpmn::Definitions {
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut def=Definitions{id:" ".to_string(),processes:Vec::new()};
    let mut proc=Process{is_executable: false, id:"default".to_string(), nodes: Vec::new(), subprocesses: Vec::new()};
    let mut node=Node {id: "default".to_string(), name: "default".to_string(), flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
    let mut text_switch = 0; //this will indicate which tag was the last if there is plain text inside the tag
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"semantic:definitions" => {let definition_attributes = parse_attributes(e);
                        //println!("attributes values: {:?}",&definition_attributes);
                        //println!("ID: {:?}",&definition_attributes[0]);
                        def.id=definition_attributes[0].clone();
                    },
                    b"semantic:process"=>{let process_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        proc.nodes.remove(0);
                        node=Node {id: "default".to_string(), name: "default".to_string(), flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
                        def.processes.push(proc);
                        println!("attributes values: {:?}", &process_attributes);
                        proc=Process{is_executable: process_attributes[0].parse::<bool>().unwrap(), id: process_attributes[1].clone(), nodes: Vec::new(), subprocesses: Vec::new()};
                    },
                    b"semantic:startEvent"=>{let node_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                            flow_object: FlowObject::Event(process_bpmn::Event::StartEvent), connections: Vec::new()};
                        println!("attributes values: {:?}",&node_attributes);
                    },
                    b"semantic:incoming"=>{
                        text_switch=1;  //set the switch to recognize the incoming connection tag
                        println!("{ }",text_switch);
                    },
                    b"semantic:outgoing"=>{
                        text_switch=2;  //set the switch to recognize the outgoing connection tag
                        println!("{ }",text_switch);
                    },
                    b"semantic:exclusiveGateway"=>{let node_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                            flow_object: FlowObject::Gateway(process_bpmn::Gateway::ExclusiveGateway), connections: Vec::new()};
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:parallelGateway"=>{let node_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                            flow_object: FlowObject::Gateway(process_bpmn::Gateway::ParallelGateway), connections: Vec::new()};
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:task"=>{let node_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                            flow_object: FlowObject::Activity(process_bpmn::Activity::Task), connections: Vec::new()};
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:userTask"=>{let node_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                            flow_object: FlowObject::Activity(process_bpmn::Activity::UserTask), connections: Vec::new()};
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:endEvent"=>{let node_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                            flow_object: FlowObject::Event(process_bpmn::Event::EndEvent), connections: Vec::new()};
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:callActivity"=>{let node_attributes = parse_attributes(e);
                        proc.nodes.push(node);
                        node=Node {id: get_id(&node_attributes), name: get_name(&node_attributes),
                            flow_object: FlowObject::Activity(process_bpmn::Activity::CallActivity), connections: Vec::new()};
                        println!("attributes values: {:?}",node_attributes);
                    },
                    b"semantic:sequenceFlow"=> {

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
                    _ => println!(),
                }
                //println!("{}", e.name().)
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

pub fn parse_subprocess(reader: &mut Reader<&[u8]>, subprocesses: &mut Vec<SubProcess>, buf: &mut Vec<u8>) {
    loop {
        match reader.read_event(buf) {
            Ok(Event::Start(ref e)) => {}
            Ok(Event::Text(e)) => {
                let text_var=e.unescape_and_decode(&reader).unwrap();
                println!("{:?}", &text_var);
            },
            Ok(Event::Eof) => {
            }, // exits the loop when reaching end of file

            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }

    }
}