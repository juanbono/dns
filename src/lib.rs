pub const TYPE_A: u16 = 1;
pub const CLASS_IN: u16 = 1;

pub fn build_query(domain_name: String, record_type: u16) -> Vec<u8> {
    let name = encode_name(domain_name);
    let id = 42;
    let recursion_desired = 1 << 8;
    let header = DnsHeader {
        id,
        num_questions: 1,
        flags: recursion_desired,
        num_answers: 0,
        num_additionals: 0,
        num_authorities: 0,
    };
    let question = DnsQuestion {
        name,
        type_: record_type,
        class_: CLASS_IN,
    };
    let mut header_bytes = header.to_bytes();
    let mut question_bytes = question.to_bytes();
    header_bytes.append(&mut question_bytes);
    header_bytes
}

fn encode_name(name: String) -> Vec<u8> {
    let mut encoded: Vec<u8> = vec![];
    for part in name.split(".") {
        encoded.push(part.len() as u8);
        encoded.append(&mut part.as_bytes().to_vec());
    }
    encoded.push(0);
    encoded
}

trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

struct DnsHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

impl ToBytes for DnsHeader {
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.id.to_be_bytes(),
            self.flags.to_be_bytes(),
            self.num_questions.to_be_bytes(),
            self.num_answers.to_be_bytes(),
            self.num_authorities.to_be_bytes(),
            self.num_additionals.to_be_bytes(),
        ]
        .concat()
    }
}

struct DnsQuestion {
    name: Vec<u8>,
    type_: u16,
    class_: u16,
}

impl ToBytes for DnsQuestion {
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.name.as_slice(),
            &self.type_.to_be_bytes(),
            &self.class_.to_be_bytes(),
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{encode_name, DnsHeader, DnsQuestion, ToBytes};

    #[test]
    fn can_convert_a_dns_header_to_bytes() {
        let header = DnsHeader {
            id: 0x1314,
            flags: 0,
            num_questions: 1,
            num_additionals: 0,
            num_authorities: 0,
            num_answers: 0,
        };

        assert_eq!(
            vec![19, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            header.to_bytes()
        )
    }

    #[test]
    fn can_encode_google_domain() {
        let encoded = encode_name(String::from("google.com"));
        assert_eq!(
            vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0],
            encoded
        )
    }
}
