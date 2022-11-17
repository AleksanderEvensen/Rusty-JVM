use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../java/MyProgram.java");

    let mut java_child = Command::new("javac")
        .args(["../java/*.java", "-source", "7", "-target", "7"])
        .spawn()
        .unwrap();
    let mut jar_child = Command::new("jar")
        .args(["cvf", "../java/MyProgram.jar", "../java/*.class"])
        .spawn()
        .unwrap();

    java_child.wait().unwrap();
    jar_child.wait().unwrap();

    std::fs::write(
        "../java-build-output.txt",
        format!(
            "===javac===\nSTDOUT:\n{:?}\n\nSTDERR:\n{:?}\n\n\n===jar===\nSTDOUT:\n{:?}\n\nSTDERR:\n{:?}",
            java_child.stdout, java_child.stderr, jar_child.stdout, jar_child.stderr
        )
        .as_bytes(),
    )
    .unwrap();
}
