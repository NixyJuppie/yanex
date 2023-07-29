use ines_header::INesHeader;

mod ines_header;

pub struct INes {
    pub header: INesHeader,
}

impl From<Vec<u8>> for INes {
    fn from(_value: Vec<u8>) -> Self {
        todo!()
    }
}
