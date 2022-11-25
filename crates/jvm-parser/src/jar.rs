use std::io::{BufReader, Read};
use std::{error::Error, fs::File, path::PathBuf};

pub struct JarFile {}

impl JarFile {
    pub fn from_file(path: &PathBuf) -> Result<JarFile, Box<dyn Error>> {
        let mut jar_archive = zip::ZipArchive::new(File::open(path)?)?;

        for i in 0..jar_archive.len() {
            let entry = jar_archive.by_index(i).unwrap();
            if entry.is_dir() {
                continue;
            }

            println!("Entry: {:?}", entry.name());

            let mut file_reader = BufReader::new(entry);
            let mut bytes = vec![];

            file_reader.read_to_end(&mut bytes)?;

            let content =
                String::from_utf8(bytes.clone()).unwrap_or(String::from("Failed to parse string"));

            println!("Content:\n\n{}", content);
            println!("\nBytes: {:?}", bytes);
        }

        println!(
            "File Names: {:?}",
            jar_archive.file_names().collect::<Vec<&str>>()
        );

        Ok(JarFile {})
    }
}
