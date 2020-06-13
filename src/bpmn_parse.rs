use super::*;
use quick_xml;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use process_bpmn;
use helper;
use process_bpmn::{Definitions, SubProcess, Process, Node, FlowObject};
use helper::{get_value_from_key};

pub fn parse_process(contents: String)->process_bpmn::Definitions {
    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut def=Definitions::default();
    let mut proc=Process::default();
    let mut node = Node::default();
    let mut text_switch = 0; //this will indicate which tag was the last if there is plain text inside the tag
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"semantic:definitions" => {
                        def.id=get_value_from_key(e, "id").unwrap();
                    },
                    b"semantic:process"=>{
                        proc.nodes.push(node);
                        proc.nodes.remove(0);
                        node=Node::default();
                        def.processes.push(proc);
                        //println!("attributes values: {:?}", &process_attributes);
                        proc=Process{is_executable: get_value_from_key(e, "isExecutable").unwrap().parse::<bool>().unwrap(),
                            id: get_value_from_key(e, "id").unwrap(), nodes: Vec::new(), subprocesses: Vec::new()};
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
                        let attr = vec![
                            get_value_from_key(e, "triggeredByEvent").unwrap(),
                            get_value_from_key(e, "startQuantity").unwrap(),
                            get_value_from_key(e, "completionQuantity").unwrap(),
                            get_value_from_key(e, "isForCompensation").unwrap(),
                            get_value_from_key(e, "id").unwrap(),
                            get_value_from_key(e, "name").unwrap(),
                        ];
                        //println!("attributes values: {:?}", &subprocess_attributes);
                        parse_subprocess(&mut reader, &mut proc.subprocesses, &mut buf, attr);
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
                        if let Ok(n) = parse_node(e) {
                            proc.nodes.push(node);
                            node = n;
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

fn parse_subprocess(reader: &mut Reader<&[u8]>, subprocesses: &mut Vec<SubProcess>, buf: &mut Vec<u8>, attr: Vec<String>) {
    let mut subproc=SubProcess{triggered_by_event: attr[0].parse::<bool>().unwrap(),
        start_quantity: attr[1].parse::<u64>().unwrap(),
        completion_quantity: attr[2].parse::<u64>().unwrap(),
        is_for_compensation: attr[3].parse::<bool>().unwrap(),
        id: attr[4].clone(), name:attr[5].clone(), nodes: Vec::new(),
        subprocesses: Vec::new(), connections: Vec::new()};
    let mut node = Node::default();
    let mut text_switch = 0;
    //let mut start_switch=false;
    loop {
        //println!("Cycle!");
        match reader.read_event(buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"semantic:incoming"=>{
                        text_switch=1;  //set the switch to recognize the incoming connection tag
                        println!("{ }",text_switch);
                    },
                    b"semantic:outgoing"=>{
                        text_switch=2;  //set the switch to recognize the outgoing connection tag
                        println!("{ }",text_switch);
                    },
                    _ => {
                        if let Ok(n) = parse_node(e) {
                            subproc.nodes.push(node);
                            node = n;
                        }
                    },
                }
            }
            Ok(Event::Text(e)) => {
                let text_var=e.unescape_and_decode(&reader).unwrap();
                match text_switch { // how do we handle the text inside the tags
                    1 => process_bpmn::add_connection(&mut node, text_switch, text_var.clone()),
                    2 => process_bpmn::add_connection(&mut node, text_switch, text_var.clone()),
                    _ =>  println!(),
                }
                println!("{:?}", &text_var);
            },
            Ok(Event::End(ref e)) => {
                if e.name() == b"semantic:subProcess" {
                    subproc.nodes.push(node);
                    subproc.connections.clone_from(&subproc.nodes[0].connections);
                    subproc.nodes.remove(0);
                    subprocesses.push(subproc);
                    break;
                }
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }

    }
}

fn parse_node(e: &BytesStart) -> Result<Node, String> {
    let node;
    //=Node {id: "default".to_string(), name: "default".to_string(), flow_object: FlowObject::Gateway(process_bpmn::Gateway::ComplexGateway), connections: Vec::new()};
    match e.name() {
        b"semantic:startEvent"=>node=parse_start_event(e),
        b"semantic:intermediateEvent"=>node=parse_intermediate_event(e),
        b"semantic:exclusiveGateway"=>node=parse_exclusive_gateway(e),
        b"semantic:parallelGateway"=>node=parse_parallel_gateway(e),
        b"semantic:task"=>node=parse_task(e),
        b"semantic:userTask"=>node=parse_user_task(e),
        b"semantic:endEvent"=> node=parse_end_event(e),
        b"semantic:callActivity"=>node=parse_call_activity(e),

        _ => {
            println!("{}", String::from_utf8_lossy(e.name()));
            return Err("Invalid node".into())
        },
    }
    Ok(node)
}

fn parse_start_event(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Event(process_bpmn::Event::StartEvent), connections: Vec::new()};
    node
}

fn parse_intermediate_event(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Event(process_bpmn::Event::IntermediateEvent), connections: Vec::new()};
    node
}

fn parse_end_event(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Event(process_bpmn::Event::EndEvent), connections: Vec::new()};
    node
}

fn parse_task(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Activity(process_bpmn::Activity::Task(
            process_bpmn::Task{completion_quantity: get_value_from_key(e, "completionQuantity").unwrap().parse::<u64>().unwrap(),
                start_quantity: get_value_from_key(e, "startQuantity").unwrap().parse::<u64>().unwrap(),
                is_for_compensation: get_value_from_key(e, "isForCompensation").unwrap().parse::<bool>().unwrap(),
            })), connections: Vec::new()};
    node
}

fn parse_user_task(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Activity(process_bpmn::Activity::UserTask(
            process_bpmn::UserTask{completion_quantity: get_value_from_key(e, "completionQuantity").unwrap().parse::<u64>().unwrap(),
            start_quantity: get_value_from_key(e, "startQuantity").unwrap().parse::<u64>().unwrap(),
            is_for_compensation: get_value_from_key(e, "isForCompensation").unwrap().parse::<bool>().unwrap(),
                implementation: get_value_from_key(e, "implementation").unwrap().clone(),
        })), connections: Vec::new()};
    node
}

fn parse_call_activity(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Activity(process_bpmn::Activity::CallActivity(process_bpmn::CallActivity{
            called_element: get_value_from_key(e, "calledElement").unwrap(),
    })), connections: Vec::new()};
    node
}

fn parse_exclusive_gateway(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Gateway(process_bpmn::Gateway::ExclusiveGateway(
            process_bpmn::ExclusiveGateway{gateway_direction: get_value_from_key(e, "gatewayDirection").unwrap()})),
        connections: Vec::new()};
    node
}

fn parse_parallel_gateway(e: &BytesStart) ->Node {
    let node=Node {id: get_value_from_key(e, "id").unwrap(),
        name: get_value_from_key(e, "name").unwrap(),
        flow_object: FlowObject::Gateway(process_bpmn::Gateway::ParallelGateway(
            process_bpmn::ParallelGateway{gateway_direction: get_value_from_key(e, "gatewayDirection").unwrap()})),
        connections: Vec::new()};
    node
}