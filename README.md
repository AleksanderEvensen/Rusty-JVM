# Java Virtual Machine in rust

_NOTE: This is only for educatinal purposes, and is not a complete implementation of oracles jvm_

It can only parse (almost) all class and jar files in the current state of the implementation.

Not All java features are implemented (obviously).

```sh
jvm.exe --path "<path_to_file>.(jar|class)"
```

Everything needed to implement, is written in thew oracle documentation:

[Oracle: The `class` File Format](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html)

[Oracle: The Java Virtual Machine Instruction Set](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html)

Goals:

-   Get an intuitive implementation that can be expanded into something that could potentially run any java program
    -   This means implementing everything when parsing the class file and also loading jar files
    -   A easy way of adding the bindings for the java methods
        -   for example: `System.out.println()` sould be mapped to rust as `println!()`. To the JVM this function is known as `java/io/PrintStream/println`. A rust macro for this should do the trick
-   Run Minecraft (lol. not happening)
