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

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: String,
    pub connection_type: ConnectionType
}

#[derive(Debug, Clone)]
pub struct ConnectionSequence {
    pub id: String,
    pub name: String,
    pub source_ref: String,
    pub target_ref: String
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
    pub subprocesses: Vec<SubProcess>,
    pub connections: Vec<ConnectionSequence>
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
    pub outer_connections: Vec<Connection>,
    pub connections: Vec<ConnectionSequence>
}

#[derive(Debug, Clone)]
pub struct Definitions {
    pub id: String,
    pub processes: Vec<Process>
}

pub fn add_connection(node: &mut Node, switch: i32, text_var: String) {
    node.connections.push(Connection{id: text_var.clone(), connection_type: ConnectionType::Incoming,
});
    //let length = node.connections.len();
    if switch==2 {
        node.connections.last_mut().unwrap().connection_type=ConnectionType::Outgoing;
    }
}

impl Default for UserTask {
    fn default() -> UserTask {
        UserTask {
            implementation: String::new(),
            start_quantity: 0,
            completion_quantity: 0,
            is_for_compensation: false,
        }
    }
}

impl Default for Task {
    fn default() -> Task {
        Task {
            start_quantity: 0,
            completion_quantity: 0,
            is_for_compensation: false,
        }
    }
}

impl Default for CallActivity {
    fn default() -> CallActivity {
        CallActivity {
            called_element: String::new()
        }
    }
}

impl Default for Process {
    fn default() -> Process {
        Process {
            is_executable: false,
            id: String::new(),
            nodes: Vec::new(),
            subprocesses: Vec::new(),
            connections: Vec::new()
        }
    }
}

impl Default for Definitions {
    fn default() -> Definitions {
        Definitions {
            id: String::new(),
            processes: Vec::new()
        }
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            id: String::new(),
            name: String::new(),
            flow_object: FlowObject::Gateway(Gateway::ComplexGateway),
            connections: Vec::new()
        }
    }
}

impl Default for Connection {
    fn default() -> Connection {
        Connection {
            id: String::new(),
            connection_type: ConnectionType::Incoming
        }
    }
}

impl Default for ExclusiveGateway {
    fn default() -> ExclusiveGateway {
        ExclusiveGateway {
            gateway_direction: "Diverging".to_string()
        }
    }
}

impl Default for ParallelGateway {
    fn default() -> ParallelGateway {
        ParallelGateway {
            gateway_direction: "Diverging".to_string()
        }
    }
}

fn get_connection_sequence(id: &str, search_space: &[ConnectionSequence])->Option<(ConnectionSequence, usize)> {
    for ( pos, sequence) in search_space.iter().enumerate() {
            if sequence.id==*id {
            return Some((sequence.clone(), pos))
        }
    }
    None
}

#[allow(dead_code)]
impl Process {
    pub fn find_connection(&self, id: &str) -> Option<ConnectionSequence> {
        let search_result=get_connection_sequence(id, self.connections.as_slice() );
        match search_result{
            Some(_) => return Some(search_result.unwrap().0),
            None =>return None
        }
    }

    pub fn find_connection_position(&self, id: &str) -> Option<usize> {
        let search_result=get_connection_sequence(id, self.connections.as_slice() );
        match search_result{
            Some(_) => return Some(search_result.unwrap().1),
            None =>return None
        }
    }
}

impl SubProcess {
    pub fn find_connection(&self, id: &str) -> Option<ConnectionSequence> {
        let search_result=get_connection_sequence(id, self.connections.as_slice() );
        match search_result{
            Some(_) => return Some(search_result.unwrap().0),
            None =>return None
        }
    }

    pub fn find_connection_position(&self, id: &str) -> Option<usize> {
        let search_result=get_connection_sequence(id, self.connections.as_slice() );
        match search_result{
            Some(_) => return Some(search_result.unwrap().1),
            None =>return None
        }
    }
}
//pub fn find_connection(id: String, )