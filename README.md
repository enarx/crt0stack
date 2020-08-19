[![Workflow Status](https://github.com/enarx/crt0stack/workflows/test/badge.svg)](https://github.com/enarx/crt0stack/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/crt0stack.svg)](https://isitmaintained.com/project/enarx/crt0stack "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/crt0stack.svg)](https://isitmaintained.com/project/enarx/crt0stack "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# crt0stack

Create the initial stack frame to start an ELF binary on Linux

## Examples

```rust
use crt0stack::{Builder, Entry};

let mut stack = [1u8; 512];
let stack = stack.as_mut();

let mut builder = Builder::new(stack);

builder.push("/init").unwrap();
let mut builder = builder.done().unwrap();

builder.push("HOME=/root").unwrap();
let mut builder = builder.done().unwrap();

let auxv = [
    Entry::Gid(1000),
    Entry::Uid(1000),
    Entry::Platform("x86_64"),
    Entry::ExecFilename("/init"),
];
auxv.iter().for_each(|e| builder.push(e).unwrap());

let handle = builder.done().unwrap();
```

License: Apache-2.0
