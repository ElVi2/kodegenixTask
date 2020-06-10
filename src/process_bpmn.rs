/*
struct StartEvent {
    id: String,
    name: String
}

struct IntermediateEvent {
    id: String,
    name: String
}

struct EndEvent {
    id: String,
    name: String
}*/

#[derive(Debug, Clone)]
pub struct UserTask {
    pub implementation: String,
    pub start_quantity: u64,
    pub completion_quantity: u64,
    pub is_for_compensation: bool,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub start_quantity: u64,
    pub completion_quantity: u64,
    pub is_for_compensation: bool,
}

#[derive(Debug, Clone)]
pub struct CallActivity {
    pub called_element: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParallelGateway {
    pub gateway_direction: String
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExclusiveGateway {
    pub gateway_direction: String
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Event {
    StartEvent,
    IntermediateEvent,
    EndEvent
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Activity {
    Task(Task),
    Transaction,
    CallActivity(CallActivity),
    UserTask(UserTask)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Gateway {
    ExclusiveGateway(ExclusiveGateway),
    EventBasedGateway,
    ParallelGateway(ParallelGateway),
    InclusiveGateway,
    ExclusiveEventBasedGateway,
    ParallelEventBasedGateway,
    ComplexGateway
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FlowObject {
    Event(Event),
    Activity(Activity),
    Gateway(Gateway)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ConnectionType {
    Incoming,
    Outgoing
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub source_ref: String,
    pub target_ref: String,
    pub connection_type: ConnectionType
}

#[derive(Debug, Clone)]
pub struct Node{
    pub name: String,
    pub id: String,
    pub flow_object: FlowObject,
    pub connections: Vec<Connection>
}

#[derive(Debug, Clone)]
pub struct Process{
    pub is_executable: bool,
    pub id: String,
    pub nodes: Vec<Node>,
    pub subprocesses: Vec<SubProcess>
}

#[derive(Debug, Clone)]
pub struct SubProcess{
    pub id: String,
    pub name: String,
    pub start_quantity: u64,
    pub completion_quantity: u64,
    pub triggered_by_event: bool,
    pub is_for_compensation: bool,
    pub nodes: Vec<Node>,
    pub subprocesses: Vec<SubProcess>,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Clone)]
pub struct Definitions {
    pub id: String,
    pub processes: Vec<Process>
}

pub fn add_connection(node: &mut Node, switch: i32, text_var: String) {
    node.connections.push(Connection{id: text_var.clone(),
        name: "default".to_string(), connection_type: ConnectionType::Incoming,
        source_ref: "default".to_string(), target_ref: "default".to_string()});
    //let length = node.connections.len();
    if switch==2 {
        node.connections.last_mut().unwrap().connection_type=ConnectionType::Outgoing;
    }
}
 /*
fn search_subprocess(id: String, cn: &mut Connection, subproc: &'_ mut SubProcess) {

}

pub fn search_connection(id: &str, proc: &'_ mut Process) -> Result<&'_ mut Connection, String> {
    for process in space {
    }
    Err("Nothing found")
}
  */