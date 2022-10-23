# Java Virtual Machine in rust

This was primarily for fun and i wanted to implement Hello World from Java Byte Code.
Not All java features are implemented (obviously), but atleast whats necessary to get "Hello World" printed on the screen

```sh
javac java/MainProgram.java # Will produce the MyProgram.class
cargo run # File path is hardcoded (may change in the future)
```

Everything needed to implement i written in thew oracle documentation:

[Oracle: The `class` File Format](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html)

[Oracle: The Java Virtual Machine Instruction Set](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html)
