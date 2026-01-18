package main

import "fmt"

func main() {
	fmt.Println("Hello, Go!")
}

/*
 * This is a simple Go program that prints "Hello, Go!" to the console.
 * It demonstrates the basic structure of a Go program with package declaration,
 * import statements, and the main function.

 # Run this program:
 go run example-01.go

 ## What does this code do?
 It prints "Hello, Go!" to the console.

## what is package main ?
 The is the name of the package. In Go, a package is a collection of Go source files that are compiled together. The special package name "main" tells the Go compiler that this file should be compiled into an executable program, rather than a library. A package named "main" must contain a main() function, which is the entry point of the program.

## what is import "fmt "
The statement imports the fmt package, which provides formatted I/O functions like Println. In Go, you import packages to use their exported functions, types, and variables.

## what is func main() ?
 The main() function is the entry point of the program. When you run a Go program, execution starts at the main() function. In this example, it calls fmt.Println() to print the message to the console.

## what is fmt.Println() ?
 The fmt.Println() function is a method provided by the fmt package that prints the given arguments to the standard output (console) followed by a newline character. In this example, it prints the string "Hello, Go!" to the console.
*/
