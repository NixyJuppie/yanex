use header::INesHeader;

mod header;

pub struct INes {
    pub header: INesHeader,
}

impl From<Vec<u8>> for INes {
    fn from(value: Vec<u8>) -> Self {
        let header_data: [u8; 16] = value[..16]
            .try_into()
            .expect("Cartridge does not contain iNES header");

        let header: INesHeader = header_data
            .try_into()
            .expect("Cartridge does not contain valid iNES header");

        Self { header }
    }
}
