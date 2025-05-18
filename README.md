# Repository
This repository holds the source code for a set of tools for a language I designed called Pine.
The tools include:
- A compiler for the Pine language
- A virtual machine implementation called pvm (Pine Virtual Machine)

# Description of Pine

The Pine language is inspired by both Rust and Python. I wanted to create an easy to use scripting language that
also includes great developer support and tooling.

# Description of PVM

Pine Virtual Machine (PVM) is the virtual machine that Pine code runs on.

## Variables in PVM

PVM variables are always size u64, and are global.

## Memory in PVM

Arrays and other dynamic objects are stored in a huge array of bytes (u8).
The alloc and dealloc instructions manage blocks of memory, and load* and store* instructions
are used to read and write memory.


# Tools that will be implemented in the future
- A package manager
- VS Code syntax highlighting

# TODO
## pvm
- other syscall ops (read, write file, etc)
- Add function support (push and pop args, call)
  - TODO need a way to save local vars before func call, look at example.pvm
- Can make a variable and label with same name, should this be allowed?
- Add failure tests
- Add comments

## Refactors
- Make functions return Results instead of panicking, so that tests and other code can inspect the errors and return better messages
  - pvm done, need to work on ast still
- Write comments
- Refactor proc macros as much as possible
  - pvm done, need to work on test_util and ast still

## Tests
- Add failure tests

## README
- Write better descriptions, move to separate docs file using typst

# Notes

I want to name everything in the language on based on the Pine theme (inspired by Rust).
- files will be called `leaves`
- modules will be called `branches`
- packages will be called `trees`
- workspaces will be called `forests`
- the package manager will be called `ranger`