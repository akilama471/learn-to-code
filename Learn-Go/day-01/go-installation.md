# Go Installation

To get started with Go, download and install it from the official website:

- Visit: https://golang.org/dl/
- Select the installer for your operating system (Windows, macOS, Linux)
- Follow the installation instructions
- Verify the installation by running `go version` in your terminal

## Setting Up Your Workspace

After installation, set up your Go workspace:

1. Create a directory for your Go projects (e.g., `~/go` or `C:\Go`)
2. Set the `GOPATH` environment variable to point to your workspace directory
3. Add `$GOPATH/bin` to your system's `PATH` variable
4. Create a `src` directory inside your workspace for source code

## what is go workspace?

A Go workspace is a directory hierarchy where Go stores its source code, compiled packages, and executable binaries. The default workspace location is `$HOME/go` on Unix-like systems and `%USERPROFILE%\go` on Windows.

The workspace contains three main subdirectories:

- `src`: Source code for your Go packages and applications
- `pkg`: Compiled package objects for your architecture and OS
- `bin`: Executable binaries for installed tools and applications

## Working with Go Modules

With Go 1.11+, Go modules are the recommended way to manage dependencies:

1. Navigate to your project directory
2. Run `go mod init <module-name>` to initialize a new module
3. Import packages using their full import path
4. Run `go build` or `go run` to automatically download dependencies
5. Use `go get <package>` to add new dependencies
