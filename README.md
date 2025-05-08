# Repository
This repository holds the source code for a set of tools for a language I designed called Pine.
The tools include:
- A compiler for the Pine language
- A virtual machine implementation called pvm (Pine Virtual Machine)

# Description of Pine

The Pine language is inspired by both Rust and Python. I wanted to create an easy to use scripting language that
also includes great developer support and tooling.

# Tools that will be implemented in the future
- A package manager
- VS Code syntax highlighting

# TODO
## Refactors
- Move tests into ast so that modules can be made private
- Make functions return Results instead of panicking, so that tests and other code can inspect the errors and return better messages
- Write comments

# Notes

I want to name everything in the language on based on the Pine theme (inspired by Rust).
- files will be called `leaves`
- modules will be called `branches`
- packages will be called `trees`
- workspaces will be called `forests`
- the package manager will be called `ranger`