use header::DnsHeader;
use question::DnsQuestion;

pub const TYPE_A: u16 = 1;
pub const CLASS_IN: u16 = 1;

mod header;
mod question;

pub fn build_query(domain_name: String, record_type: u16) -> Vec<u8> {
    let name = encode_name(domain_name);
    let id = 42;
    let recursion_desired = 1 << 8;
    let header = DnsHeader::new(id, recursion_desired, 1, 0, 0, 0);
    let question = DnsQuestion::new(name, record_type, CLASS_IN);
    let mut header_bytes = header.to_bytes();
    let mut question_bytes = question.to_bytes();
    header_bytes.append(&mut question_bytes);
    header_bytes
}

fn encode_name(name: String) -> Vec<u8> {
    let mut encoded: Vec<u8> = vec![];
    // prefix the length to each part.
    for part in name.split(".") {
        encoded.push(part.len() as u8);
        encoded.append(&mut part.as_bytes().to_vec());
    }
    // add a 0x00 at the end.
    encoded.push(0);
    encoded
}

trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[cfg(test)]
mod tests {
    use crate::encode_name;

    #[test]
    fn can_encode_google_domain() {
        let encoded = encode_name(String::from("google.com"));
        assert_eq!(
            vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0],
            encoded
        )
    }
}
