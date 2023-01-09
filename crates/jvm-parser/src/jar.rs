use std::collections::HashMap;
use std::io::Read;
use std::{error::Error, fs::File, path::PathBuf};

use binary_reader::BinaryReader;
use flate2::read::DeflateDecoder;

use crate::classfile::ClassFile;

#[derive(Debug, Default)]
pub struct JarFile {
    manifest: JarManifest,
    classes: HashMap<String, ClassFile>,
}

#[derive(Debug, Default)]
pub struct JarManifest {
    pub version: Option<String>,
    pub created_by: Option<String>,
    pub main_class: Option<String>,
}

impl JarManifest {
    pub fn from_deflated_bytes(bytes: &Vec<u8>) -> JarManifest {
        let mut decoder = DeflateDecoder::new(bytes.as_slice());
        let mut data = String::new();
        decoder.read_to_string(&mut data).unwrap();

        let mut manifest = JarManifest {
            ..Default::default()
        };

        data.lines().for_each(|line| {
            if line.trim() == "" {
                return;
            }

            if let Some((attribute, value)) = line.split_once(":") {
                let attribute = attribute.trim();
                let value = value.trim().to_string();

                match attribute {
                    "Manifest-Version" => manifest.version = Some(value.to_string()),
                    "Created-By" => manifest.created_by = Some(value.to_string()),
                    "Main-Class" => manifest.main_class = Some(value.to_string()),

                    _ => todo!("Implement manifest parsing for key: '{}'", attribute),
                };
            } else {
                println!("Failed to parse line: {:?}", line)
            }
        });

        return manifest;
    }
}

impl JarFile {
    pub fn from_file(path: &PathBuf) -> Result<JarFile, Box<dyn Error>> {
        let mut jar_reader = BinaryReader::from_file(&mut File::open(path)?)?;

        let central_dir_file_header = vec![0x50, 0x4B, 0x01, 0x02];

        let mut jar_file = JarFile {
            manifest: JarManifest::default(),
            classes: HashMap::new(),
        };

        while let Ok(offset) = jar_reader.find_next(&central_dir_file_header) {
            jar_reader.move_to(offset);

            let compressed_size = *jar_reader.jump(20).read::<u32>()? as usize;
            let file_name_length = *jar_reader.jump(4).read::<u16>()? as usize;

            // The file offset + the fields we don't care about
            let file_data_offset = *jar_reader.jump(12).read::<u32>()? as usize + 28;

            // Read the file name
            let file_name = jar_reader.read_string(file_name_length)?;

            // Store the offset, that way we can move back
            let old_offset = jar_reader.get_current_offset();

            // Move to the file entry
            jar_reader.move_to(file_data_offset);

            // Read the amount of extra fields
            let extra_field_length = *jar_reader.read::<u16>()? as usize;

            // Read the deflated bytes
            let data = jar_reader
                .jump(file_name_length + extra_field_length)
                .read_bytes(compressed_size as usize)?;

            // Go back to the Central Dir Entry
            jar_reader.move_to(old_offset);

            if file_name == "META-INF/MANIFEST.MF" {
                jar_file.manifest = JarManifest::from_deflated_bytes(&data);
            } else if file_name.ends_with(".class") {
                let mut decoder = DeflateDecoder::new(data.as_slice());
                let mut buffer = vec![];

                decoder.read_to_end(&mut buffer).unwrap();

                let class_file = ClassFile::from_bytes(buffer).unwrap();

                let class = class_file
                    .constant_pool
                    .get_class_at(class_file.this_class)
                    .unwrap();

                let name = class_file
                    .constant_pool
                    .get_utf8_at(class.name_index)
                    .unwrap()
                    .data
                    .clone();
                println!("Adding the class '{}' to the class map", name);
                jar_file.classes.insert(name, class_file);
            } else {
                println!("Ignore: {}", file_name);
            }
        }
        Ok(jar_file)
    }
}
