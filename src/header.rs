use crate::ToBytes;

pub(crate) struct DnsHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

impl DnsHeader {
    pub fn new(
        id: u16,
        flags: u16,
        num_questions: u16,
        num_answers: u16,
        num_authorities: u16,
        num_additionals: u16,
    ) -> Self {
        Self {
            id,
            flags,
            num_additionals,
            num_answers,
            num_authorities,
            num_questions,
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::{header::DnsHeader, ToBytes};

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
}
