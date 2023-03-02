use std::collections::HashMap;
use std::io::Read;
use std::{error::Error, fs::File, path::PathBuf};

use byte_reader::ByteReader;
use flate2::read::DeflateDecoder;
use rayon::prelude::*;

use crate::classfile::JavaClass;

#[derive(Debug, Default)]
pub struct JarFile {
    pub manifest: JarManifest,
    pub classes: HashMap<String, JavaClass>,
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

        JarManifest::from_string(&data)
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> JarManifest {
        JarManifest::from_string(&String::from_utf8(bytes.clone()).unwrap())
    }

    pub fn from_string(manifest_content: &String) -> JarManifest {
        let mut manifest = JarManifest {
            ..Default::default()
        };

        manifest_content.lines().for_each(|line| {
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

                    _ => {} //println!("Implement manifest parsing for key: '{}'", attribute),
                };
            } else {
                // println!("Failed to parse line: {:?}", line)
            }
        });

        return manifest;
    }
}

impl JarFile {
    pub fn from_file(path: &PathBuf) -> Result<JarFile, Box<dyn Error>> {
        let mut jar_reader = ByteReader::from_file(&mut File::open(path)?)?;

        let central_dir_file_header = vec![0x50, 0x4B, 0x01, 0x02];

        let mut jar_file = JarFile {
            manifest: JarManifest::default(),
            classes: HashMap::new(),
        };

        let cdr_offsets: Vec<usize> =
            jar_reader.find_all_offsets_parallel(&central_dir_file_header);

        let java_class_bytes: Vec<Vec<u8>> = cdr_offsets
            .iter()
            .filter_map(|file_offset| {
                jar_reader.move_to(*file_offset);

                let (file_name, bytes) = read_cdr_file_bytes(&mut jar_reader).unwrap();

                if file_name == "META-INF/MANIFEST.MF" {
                    jar_file.manifest = JarManifest::from_bytes(&bytes);
                    return None;
                } else if file_name.ends_with(".class") {
                    return Some(bytes);
                } else {
                    return None;
                }
            })
            .collect();

        jar_file.classes = java_class_bytes
            .into_par_iter()
            .map(|bytes| {
                let java_class = JavaClass::from_bytes(&bytes).unwrap();

                let class = java_class
                    .constant_pool
                    .get_class_at(java_class.this_class)
                    .unwrap();

                let name = java_class
                    .constant_pool
                    .get_utf8_at(class.name_index)
                    .unwrap()
                    .data
                    .clone();

                (name, java_class)
            })
            // .inspect(|(class, _)| println!("The class '{class}' was parsed"))
            .collect();

        Ok(jar_file)
    }
}

fn read_cdr_file_bytes(reader: &mut ByteReader) -> std::io::Result<(String, Vec<u8>)> {
    let compressed_size = reader.jump(20).read::<u32>()? as usize;

    let file_name_length = reader.jump(4).read::<u16>()? as usize;

    // The file offset + the fields we don't care about
    let file_data_offset = reader.jump(12).read::<u32>()? as usize;

    // Read the file name
    let file_name = reader.read_string(file_name_length)?;

    // Move to the file entry
    reader.move_to(file_data_offset);

    let compression_method = reader.jump(8).read::<u16>()? as usize;

    // Read the amount of extra fields
    let extra_field_length = reader.jump(18).read::<u16>()? as usize;

    // Read the deflated bytes
    let data = reader
        .jump(file_name_length + extra_field_length)
        .read_bytes(compressed_size)?;

    let data = match compression_method {
        0 => data,

        8 => {
            let mut decoder = DeflateDecoder::new(data.as_slice());
            let mut buffer = vec![];
            decoder.read_to_end(&mut buffer).unwrap();
            buffer
        }
        not_impl_comp_method => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("The the compression method {not_impl_comp_method} is not implemented yet, consider adding it"),
            ));
        }
    };

    Ok((file_name, data))
}

impl JarFile {
    pub fn get_main_class(&self) -> Option<&JavaClass> {
        let Some(main_class) = &self.manifest.main_class else {
        	return None;
        };
        self.classes.get(main_class)
    }
}
