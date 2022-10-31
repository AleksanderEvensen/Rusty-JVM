extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use std::{collections::VecDeque, path::PathBuf};

#[proc_macro]
pub fn generate_mappings(input: TokenStream) -> TokenStream {
    let inputs: Vec<TokenTree> = input.into_iter().collect();

    if inputs.len() < 2 {
        return TokenStream::new();
    }

    let mut iter = inputs.iter();

    let value = iter.next().unwrap();
    iter.next();
    let function = iter.next().unwrap();
    iter.next();
    let path = iter.next().unwrap();

    let mappings = read_dir_recursive(PathBuf::from(path.to_string().replace("\"", "")));

    let output = format!(
        "{}\nmatch {} {{ \n{}\nunknown => panic!(\"Java Class ({{}}) is not implemented\", unknown), }}",
        mappings
            .iter()
            .map(|v| format!("use {}::{};", v.0.replace("/", "::"), v.1))
            .collect::<Vec<String>>()
            .join("\n"),
        value,
        mappings
            .iter()
            .map(|v| format!("\"{}\" => {}::{},", v.0, v.1, function))
            .collect::<Vec<String>>()
            .join("\n"),
    );

    format!("{{ {} }}", output).parse().unwrap()
}

fn read_dir_recursive(root_path: PathBuf) -> Vec<(String, String)> {
    let mut paths: Vec<(String, String)> = vec![];

    let mut queue: VecDeque<PathBuf> = vec![].into();

    if let Ok(entries) = std::fs::read_dir(&root_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();
                if metadata.is_dir() {
                    queue.push_front(entry.path())
                }
            }
        }
    }

    while let Some(path) = queue.pop_back() {
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata().unwrap();
                    if metadata.is_dir() {
                        queue.push_front(entry.path())
                    } else if metadata.is_file() {
                        let name = String::from(entry.file_name().to_string_lossy());
                        if name != "mod.rs".to_string() && name.ends_with(".rs") {
                            let namespace = path
                                .to_string_lossy()
                                .replace("\\\\", "/")
                                .replace("\\", "/")
                                .replace(
                                    (String::from(root_path.clone().to_string_lossy()) + "/")
                                        .as_str(),
                                    "",
                                );
                            let class_name = name.replace(".rs", "");
                            paths.push((format!("{}/{}", namespace, class_name), class_name));
                        }
                    }
                }
            }
        }
    }

    paths
}
