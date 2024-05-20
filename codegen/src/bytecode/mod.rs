use value::Value;

pub mod io;

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
    Dup,
    StoreAddr,
    BindAddr(String),
    FreeAddr,
    Call(String, usize), // function name and number of arguments
    CallProc(String),
    CallFn(String),
}
