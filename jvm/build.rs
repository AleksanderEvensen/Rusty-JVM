use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../java/*.java");

    let mut java_child = Command::new("javac")
        .args(["../java/*.java", "-source", "7", "-target", "7"])
        .spawn()
        .unwrap();

    java_child.wait().unwrap();

    let mut jar_child = Command::new("jar")
        .args(["cvf", "../java/MyProgram.jar", "../java/*.class"])
        .spawn()
        .unwrap();

    jar_child.wait().unwrap();

    std::fs::write(
        "../java-build-output.txt",
        format!(
            "===javac===\nSTDOUT:\n{:?}\n\nSTDERR:\n{:?}\n\n\n===jar===\nSTDOUT:\n{:?}\n\nSTDERR:\n{:?}",
            java_child.stdout.take(), java_child.stderr.take(), jar_child.stdout.take(), jar_child.stderr.take()
        )
        .as_bytes(),
    )
    .unwrap();
}
