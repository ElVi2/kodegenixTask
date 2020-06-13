use quick_xml;
use quick_xml::events::BytesStart;

#[allow(dead_code)]
pub fn parse_attributes(e: &BytesStart)->Vec<String> {
    let definition_attributes = e.attributes().map(|a| {
        let attr = a.unwrap();

        String::from_utf8_lossy(&attr.value).into_owned()
    })
        .collect::<Vec<_>>();
    definition_attributes
}

pub fn get_value_from_key(e: &BytesStart, key: &str) ->Result<String, String> {
    let key_u8=key.as_bytes();
    for attr in e.attributes() {
        let unwrapped=attr.unwrap().clone();
        if unwrapped.key==key_u8 {
            return Ok(String::from_utf8_lossy(&unwrapped.value).into_owned());
        }
        //println!("{:?}", attr.unwrap());
    }
    Err("Not found".to_string())
}