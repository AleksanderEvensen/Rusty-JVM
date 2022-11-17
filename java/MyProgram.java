package com.github.aleks.rusty;

public class MyProgram {
	public static void main(String[] args) {
		MyProgram.print_string("Hello World");
	}

	public static native void print_string(String string);
}