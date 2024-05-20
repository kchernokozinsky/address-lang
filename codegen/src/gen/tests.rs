use common::util::read_file;

#[cfg(test)]
use super::*;

// 5 + 3
#[test]
fn test_visit_binary_op() {
    let source_text = read_file("test/add.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let mut generator = BytecodeGenerator::new(&algo);
    generator.visit_algorithm(&algo);

    let bytecode = generator.get_bytecode();
    println!("{:?}", bytecode);
    assert_eq!(
        bytecode,
        vec![
            Bytecode::Constant(Value::new_int(5)),
            Bytecode::Constant(Value::new_int(3)),
            Bytecode::Add
        ]
    );
}

// D {list1, 4}
#[test]
fn test_visit_mulderef_op() {
    let source_text = read_file("test/mulderef.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let mut generator = BytecodeGenerator::new(&algo);
    generator.visit_algorithm(&algo);

    let bytecode = generator.get_bytecode();
    println!("{:?}", bytecode);
    assert_eq!(
        bytecode,
        vec![
            Bytecode::LoadVar("list1".to_string()),
            Bytecode::Constant(Value::Int(4)),
            Bytecode::MulDeref
        ]
    );
}

#[test]
fn test_visit_deref_op() {
    let source_text = read_file("test/deref.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let mut generator = BytecodeGenerator::new(&algo);
    generator.visit_algorithm(&algo);

    let bytecode = generator.get_bytecode();
    println!("{:?}", bytecode);
    assert_eq!(
        bytecode,
        vec![Bytecode::Constant(Value::Int(4)), Bytecode::Deref]
    );
}

#[test]
fn test_visit_negate_op() {
    let source_text = read_file("test/negate.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let mut generator = BytecodeGenerator::new(&algo);
    generator.visit_algorithm(&algo);

    let bytecode = generator.get_bytecode();
    println!("{:?}", bytecode);
    assert_eq!(
        bytecode,
        vec![Bytecode::Constant(Value::new_bool(true)), Bytecode::Negate]
    );
}
