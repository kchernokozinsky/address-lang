use value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Bytecode {
    Halt,
    Return,
    Constant(Value), // Example: Constant values, can be extended to other types
    Not,
    And,
    Or,
    Negate,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    Greater,
    Less,
    Pop,
    Jump(usize),
    JumpIfFalse(usize),
    Send,
    Deref,
    MulDeref,
    SetVar(String),
    GetVar(String),
    Alloc,
    Call(String, usize), // function name and number of arguments
    CallProc(String),
    CallFn(String),
}
