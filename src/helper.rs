use quick_xml;
use quick_xml::events::BytesStart;

pub fn parse_attributes(e: &BytesStart)->Vec<String> {
    let definition_attributes = e.attributes().map(|a| {
        let attr = a.unwrap();

        String::from_utf8_lossy(&attr.value).into_owned()
    })
        .collect::<Vec<_>>();
    definition_attributes
}

pub fn get_name(v: &Vec<String>) -> String {
    let length=v.len();
    v[length-2].clone()
}

pub fn get_id(v: &Vec<String>) -> String {
    let length=v.len();
    v[length-1].clone()
}