# rs-args
![Static Badge](https://img.shields.io/badge/crates-io-f66a00?link=https%3A%2F%2Fcrates.io%2Fcrates%2Frs-args)
***
## Overview
A simple, easy to use command argument parser for Rust.

## Installation
1. Initialise a cargo project ( you can skip this if you already have one )
<div style="display: flex; justify-content: space-between; padding: 0 50px 0 50px;">
<div>

```bash
cargo init
```
</div>
<div style="padding-top: 10px;">
    <span style="color: #21ba79; font-weight: 700; margin-right: 4px">Created</span>binary (application) package
</div>
</div>

2. Add `rs-args` as a dependency
<div style="padding: 0 50px 0 50px;">

```toml
rs-args = "<latest version>"
```
</div>

## Usage
Now that you installed `rs-args` in your cargo project, you have
access to the `rs_args` module. This allows you to automatically parse arguments.

You can call
```rust
let args = rs_args::parse_arguments();
```

to retrieve the program arguments and parse them automaticaly, or
you can call
```rust
rs_args::get_named(...) 
// or
rs_args::get_positional(...)
// etc.
```

to directly retrieve the argument values.