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
## ast
- Need to insert return statement if void function has none
  - Need another semantic pass for checking that all code paths return if function is non void, and inserts returns for void functions
  - May need to do this with basic blocks?
- Add failure tests
- Finish parsing the rest of the Pine constructs
  - function calls
  - polymorphic types
  - arrays
  - objects
  - interfaces
  - enums
  - iterators
  - collections
    - lists
    - sets
    - maps
    - tuples
  - range objects
  - for loops
  - pattern matching
  - module imports
    - module imports should be resolved in a semantic pass
  - public access modifier (or export)
  - make it so (expr).function() == function(expr)

## sem
- add better type inference
  - Implement Hindley-Milner algorithm
- any other semantic passes?
- add tests

## gen
- Will need to collect a set of local var symbols for each function call, then save and restore

## pvm
- May want to implement functions with separate local variables, makes codegen easier
- other syscall ops (read and write file, etc)
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