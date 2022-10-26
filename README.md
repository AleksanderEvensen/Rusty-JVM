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

Goals:

-   Get an intuitive implementation that can be expanded into something that could potentially run any java program
    -   This means implementing everything when parsing the class file and also loading jar files
    -   A easy way of adding the bindings for the java methods
        -   for example: `System.out.println()` sould be mapped to rust as `println!()`. To the JVM this function is known as `java/io/PrintStream/println`. A rust macro for this should do the trick
-   Run Minecraft (lol. not happening)
