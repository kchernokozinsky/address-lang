use value::Value;

pub mod serializer;

#[derive(Debug, PartialEq, Clone)]
pub enum Bytecode {
    Halt,
    Return,
    Constant(Value),
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
    NotEqual,
    Greater,
    Less,
    Pop,
    Label(String),
    Jump(usize),
    JumpIfFalse(usize),
    Deref,
    MulDeref,
    StoreVar(String),
    LoadVar(String),
    Store,
    Alloc,
    AllocMany(usize),
    Dup,
    StoreAddr, // store to the address and push to the stack
    BindAddr(String),
    FreeAddr,
    CallBuiltin(String, usize),   // function name and number of arguments
    CallSubProgram(usize, usize), // label, arity, label_to
    PushScope,                    // Push a new scope to the stack
    PopScope,
    Swap, //
}
