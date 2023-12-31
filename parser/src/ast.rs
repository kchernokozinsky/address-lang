
#[derive(Clone,Debug)]
pub enum Algorithm {
    Body{statements: Vec<Statement>},
}

#[derive(Clone,Debug)]
pub enum DeclarationType {
    Const,
    Var,
}

#[derive(Clone,Debug)]
pub enum Statement {
    Declare{lhs: Expression, rhs: Expression, dt: DeclarationType},
    Assign{lhs: Expression, rhs: Expression},
    Replace{lhs: Expression, rhs: Expression},
    Expression{expression: Expression},
    Exit
    
}

#[derive(Clone,Debug)]
pub enum Expression {
    Int{value: i64},
    Var{name: String},
    Call{function: String, args: Vec<Box<Expression>>},
    UnaryOp{op: UnaryOp, expr: Box<Expression>},
    BinaryOp{op: BinaryOp, lhs: Box<Expression>, rhs: Box<Expression>},
}

#[derive(Clone,Debug)]
pub enum BinaryOp {
    EQ,
    NE,
    GT,
    LT,
    
    Sum,
    Sub,
    Mul,
    Div,
    Mod,

    And,
    Or,
}

#[derive(Clone,Debug)]
pub enum UnaryOp {
    Dereference,
    Not,
}