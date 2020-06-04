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
    Task,
    SubProcess,
    Transaction,
    CallActivity,
    UserTask
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Gateway {
    ExclusiveGateway,
    EventBasedGateway,
    ParallelGateway,
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
    pub nodes: Vec<Node>
}

#[derive(Debug, Clone)]
pub struct Definitions {
    pub id: String,
    pub processes: Vec<Process>
}
