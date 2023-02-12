package com.ahse.jvm;

public class Main {
	public static void main(String[] args) {		
		Main.print("Hello World, from Java");
	}
	public static native void print(String msg);
}
