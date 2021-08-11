use anyhow::{Result, Context};
use binary_reader::{BinaryReader, Endian};
use std::fs::File;

#[derive(Default)]
pub struct GdpHeader {
    pub patch_info: String, // 260
    pub file_count: u32,
}

#[derive(Default)]
pub struct GdpEntry {
    pub file_path: String, // 264
    pub stream_position: i64,
    pub file_size: i64,
}

pub struct GdpFile {
    pub header: GdpHeader,
    pub entries: Vec<GdpEntry>,
    pub reader: BinaryReader
}

impl GdpFile {
    pub fn open(path: &str) -> Result<GdpFile> {
        let mut file = File::open(path).context("invalid gdp path supplied")?;
        let mut reader = BinaryReader::from_file(&mut file);
        reader.set_endian(Endian::Little);

        let mut header = GdpHeader::default();

        reader.adv(8);
        header.patch_info = reader.read_cstr();
        reader.jmp(272);
        header.file_count = reader.read_u32()?;
        reader.adv(40);

        let mut entries: Vec<GdpEntry> = Vec::new();

        for ii in 0..header.file_count {
            entries.insert(ii as usize, GdpEntry::default());
            let mut entry = entries.get_mut(ii as usize).unwrap();

            reader.adv(8);
            entry.file_path = reader.read_cstr();
            reader.adv(263 - entry.file_path.len());
            entry.stream_position = reader.read_i64()?;
            entry.file_size = reader.read_i64()?;
            reader.adv(28);
        }

        Ok(Self {
            header,
            entries,
            reader
        })
    }

    pub fn extract(&mut self, index: u32) -> Result<Vec<u8>> {
        let entry = &self.entries[index as usize];

        self.reader.jmp(entry.stream_position as usize);
        let t = self.reader.read(entry.file_size as usize).context(format!("failed to read gdp file at index {}", index))?;

        Ok(t.to_vec())
    }
}
