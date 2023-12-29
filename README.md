# Readme ( rs-args )
`rs-args` is a rust crate which enables developers to easily create
command-line-interfaces using just rust. It enables command argument
parsing by using the tools of a full-blown programming language. It is
also actively being maintained as part of my "suite" of tools which
makes my, and hopefully your, life as a software developer much easier.

I am aware that there are probably much simpler solutions to this
problem, but I decided to approach this my own way. 

## Implementation
Currently, this isn't quite done, so you can't really use it yet, but
you can piece together the lexer and parse by yourself and use that to
parse your arguments.

### Step 1 - Lexical Analysis
We first need to convert an input. This will be the command but wihout
the program name ( `cargo init` -> `init`)

```rust
let input = "-key=value positional" // This should ( in theory ) produce
                                    // two arguments: one named and one positional
```

Then, we need to convert that plain text into a more machine readable format - tokens.

```rust
let tokens = rs_args::lexing::tokenize(String::from(input)).unwrap();
```

### Step 2 - Syntactical Analysis
This is where the actual magic happens. We can easily convert these tokens into arguments by doing as follows.
```rust
let arguments = rs_args::parsing::parse(tokens).unwrap();
```

## Contact
If you have suggestions or complaints, feel free to open an issue or
create a pull request, and I'll look into it.