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
    CallActivity
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

#[derive(Debug, Clone)]
pub enum ConnectionType {
    Ingoing,
    Outgoing
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub start: String,
    pub end: String
}

#[derive(Debug, Clone)]
pub struct Node{
    pub flow_object: FlowObject,
    pub oonnections: Connection
}

#[derive(Debug, Clone)]
pub struct Process{
    pub is_executable: bool,
    pub id: String,
    pub nodes: Vec<FlowObject>
}

#[derive(Debug, Clone)]
pub struct Definitions {
    pub id: String,
    pub processes: Vec<Process>
}
