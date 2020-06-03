#[allow(dead_code)]
#[derive(Debug)]
pub enum Event {
    StartEvent,
    IntermediateEvent,
    EndEvent
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Activity {
    Task,
    SubProcess,
    Transaction,
    CallActivity
}

#[allow(dead_code)]
#[derive(Debug)]
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
#[derive(Debug)]
pub enum FlowObject {
    Event(Event),
    Activity(Activity),
    Gateway(Gateway)
}

#[derive(Debug)]
pub struct Process{
    pub nodes: Vec<FlowObject>
}

#[derive(Debug)]
pub struct Definitions {
    pub id: String,
    pub processes: Vec<Process>
}
