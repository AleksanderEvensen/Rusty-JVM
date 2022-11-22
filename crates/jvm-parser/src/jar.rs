use std::{error::Error, path::PathBuf};

use binary_reader::BinaryReader;

use crate::utils::little_endian::{read_bytes, read_u2, read_u4};

pub struct JarFile {}

impl JarFile {
    pub fn from_file(path: &PathBuf) -> Result<JarFile, Box<dyn Error>> {
        JarFile::from_bytes(std::fs::read(path)?)
    }

    pub fn from_bytes(mut bytes: Vec<u8>) -> Result<JarFile, Box<dyn Error>> {
        let mut jar = JarFile {};

        let mut jar_reader = BinaryReader::from_vec(&bytes);
        let offset = jar_reader.find(vec![0x50, 0x4B, 0x05, 0x06]).unwrap();
        let header = jar_reader.move_to(offset).read_u32()?;

        let number_of_disks = jar_reader.read_u16()?;
        let cd_entries = jar_reader.jump(2).read_u16()?;
        let cd_entries_offset = jar_reader.jump(6).read_u32()?;

        println!("CD Header: {:X?}", header);
        println!("Number of Disks: {}", number_of_disks);
        println!("Total Entries: {}", cd_entries);
        println!("Entry Offset: {:X?}", cd_entries_offset);
        println!("");
        jar_reader.move_to(cd_entries_offset as usize);

        for _ in 0..cd_entries {
            println!("==File==");
            let file_header = jar_reader.read_u32()?;
            let bit_flags = jar_reader.jump(4).read_u16()?;
            let compression = jar_reader.read_u16()?;
            let compressed_size = jar_reader.jump(8).read_u32()?;
            let file_name_length = jar_reader.jump(4).read_u16()?;
            let extra_field_length = jar_reader.read_u16()?;
            let file_comment_length = jar_reader.read_u16()?;
            let file_rel_offset = jar_reader.jump(8).read_u32()?;
            let file_name = jar_reader.read_string(file_name_length as usize)?;
            jar_reader
                .jump(extra_field_length as usize)
                .jump(file_comment_length as usize);

            println!("File Header: {:X?}", file_header);
            println!("Bit Flags: {:#b}", bit_flags);
            println!("Compression: {}", compression);
            println!("Compressed Size: {}", compressed_size);
            println!("File relative offset: {:X?}", file_rel_offset);
            println!("File Name: {}", file_name);
            println!();
        }

        Ok(jar)
    }
}
