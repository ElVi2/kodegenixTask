#[allow(dead_code)]
pub enum Event {
    StartEvent,
    IntermediateEvent,
    EndEvent
}

#[allow(dead_code)]
pub enum Activity {
    Task,
    SubProcess,
    Transaction,
    CallActivity
}

#[allow(dead_code)]
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
pub enum FlowObject {
    Event(Event),
    Activity(Activity),
    Gateway(Gateway)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Semantic;

#[derive(Serialize, Deserialize, Debug)]
pub struct Definitions {
    pub id: String,
    #[serde(rename="process")]
    pub semantic_process: Vec<String>
}