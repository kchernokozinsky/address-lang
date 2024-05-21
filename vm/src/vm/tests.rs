#[cfg(test)]
use super::*;

#[test]
fn test_vm_execution() {
    let bytecode = vec![
        Bytecode::Constant(Value::new_int(5)),
        Bytecode::BindAddr("x".to_string()),
        Bytecode::LoadVar("x".to_string()),
        Bytecode::Constant(Value::new_int(3)),
        Bytecode::Add,
        Bytecode::BindAddr("y".to_string()),
        Bytecode::LoadVar("y".to_string()),
        Bytecode::Halt,
    ];

    let mut vm = VM::new(bytecode);
    vm.run();

    assert_eq!(vm.stack.pop().unwrap(), Value::new_int(8));
}

#[test]
fn test_alloc() {
    let bytecode = vec![
        Bytecode::Alloc, // Allocate a new address
        Bytecode::Halt,
    ];

    let mut vm = VM::new(bytecode);
    vm.run();

    assert_eq!(vm.stack.pop().unwrap(), Value::new_int(0)); // The first allocated address should be 0
}

#[test]
fn test_free_addr() {
    let bytecode = vec![
        Bytecode::Constant(Value::new_int(10)),
        Bytecode::StoreAddr, // Store the value and push the address
        Bytecode::FreeAddr,  // Free the address
        Bytecode::Alloc,     // Allocate a new address (should reuse the freed address)
        Bytecode::Halt,
    ];

    let mut vm = VM::new(bytecode);
    vm.run();

    assert_eq!(vm.stack.pop().unwrap(), Value::new_int(0)); // The first allocated address should be reused
}

#[test]
fn test_deref_unallocated_address() {
    let bytecode = vec![
        Bytecode::Constant(Value::new_int(42)),
        Bytecode::StoreAddr, // Store a value and push the address
        Bytecode::Constant(Value::new_int(100)), // Arbitrary address that is not allocated
        Bytecode::Deref,     // Dereference the unallocated address
        Bytecode::Halt,
    ];

    let mut vm = VM::new(bytecode);
    vm.run();

    assert_eq!(vm.stack.pop().unwrap(), Value::Null); // Dereferencing unallocated address should return null
}

#[test]
fn test_alloc_many() {
    let bytecode = vec![
        Bytecode::AllocMany(3), // Allocate 3 consecutive addresses
        Bytecode::Halt,
    ];

    let mut vm = VM::new(bytecode);
    vm.run();

    assert_eq!(vm.stack.len(), 3);
    assert_eq!(vm.stack[0], Value::new_int(0));
    assert_eq!(vm.stack[1], Value::new_int(1));
    assert_eq!(vm.stack[2], Value::new_int(2));
}

#[test]
fn test_alloc_many_with_existing_addresses() {
    let bytecode = vec![
        Bytecode::Alloc,        // Allocate 1 address
        Bytecode::FreeAddr,     // Free the address
        Bytecode::AllocMany(3), // Allocate 3 consecutive addresses
        Bytecode::Halt,
    ];

    let mut vm = VM::new(bytecode);
    vm.run();

    assert_eq!(vm.stack.len(), 3);
    assert_eq!(vm.stack[0], Value::new_int(1));
    assert_eq!(vm.stack[1], Value::new_int(2));
    assert_eq!(vm.stack[2], Value::new_int(3));
}