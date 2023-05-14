use crate::ToBytes;

pub(crate) struct DnsQuestion {
    name: Vec<u8>,
    type_: u16,
    class_: u16,
}

impl DnsQuestion {
    pub fn new(name: Vec<u8>, type_: u16, class_: u16) -> Self {
        Self {
            name,
            type_,
            class_,
        }
    }
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
