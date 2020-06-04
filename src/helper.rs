//#[macro_use]
use super::*;
use quick_xml;
use quick_xml::events::BytesStart;
use process_bpmn;
use process_bpmn::{Node, Connection, ConnectionType };

pub fn parse_attributes(e: &BytesStart)->Vec<String> {
    let definition_attributes = e.attributes().map(|a| {
        let attr = a.unwrap();

        String::from_utf8_lossy(&attr.value).into_owned()
    })
        .collect::<Vec<_>>();
    definition_attributes
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