use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=java/com/");

    println!("Clearing build dir");
    std::fs::remove_dir_all("../java/out").unwrap();

    let java_child = Command::new("javac")
        .args([
            "-sourcepath",
            "../java/",
            "-d",
            "../java/out",
            "../java/com/ahse/jvm/Main.java",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    let jar_child = Command::new("jar")
        .current_dir("../java/out")
        .args(["cfev", "../jvm-test.jar", "com.ahse.jvm.Main", "*"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let java_output = String::from_utf8(java_child.stdout).unwrap();
    let java_error = String::from_utf8(java_child.stderr).unwrap();
    let jar_output = String::from_utf8(jar_child.stdout).unwrap();
    let jar_error = String::from_utf8(jar_child.stderr).unwrap();

    std::fs::write(
        "../java-build-output.txt",
        format!(
            "===javac===\nSTDOUT:\n\n{}\n\nSTDERR:\n\n{}\n\n\n===jar===\nSTDOUT:\n\n{}\n\nSTDERR:\n\n{}",
            java_output, java_error, jar_output, jar_error
        )
        .as_bytes(),
    )
    .unwrap();
}
