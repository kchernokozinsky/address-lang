# VM Library

This library implements a virtual machine for executing a custom bytecode.

## Features

- Stack-based VM
- Support for various bytecode operations
- Scopes and variable binding
- Built-in functions
- Logging support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
vm = { path = "path/to/your/vm/library" }
```
## Example

``` use vm::{VM, Bytecode, Value};
fn main() {
    let bytecode = vec![
        Bytecode::Constant(Value::new_int(42)),
        Bytecode::StoreVar("x".to_string()),
        Bytecode::LoadVar("x".to_string()),
        Bytecode::Add,
        Bytecode::Return,
        Bytecode::Halt,
    ];

    let mut vm = VM::new(bytecode);
    vm.run();
}
```
