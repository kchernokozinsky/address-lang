#[derive(Clone, Debug)]
pub enum Algorithm {
    Body { lines: Vec<FileLine> },
}

#[derive(Clone, Debug)]
pub enum FileLine {
    Line {
        labels: Vec<String>,
        statements: Statements,
    },
}

impl FileLine {
    pub fn labels(&self) -> &Vec<String> {
        match self {
            FileLine::Line { labels, .. } => labels,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statements {
    OneLineStatement(OneLineStatement),
    SimpleStatements(Vec<SimpleStatement>),
}

#[derive(Clone, Debug)]
pub enum OneLineStatement {
    Loop {
        initial_value: Expression,
        step: Expression,
        last_value: Expression,
        label_until: String,
        label_to: String,
    },
    Predicate {
        condition: Expression,
        if_true: Box<Statements>,
        if_false: Box<Statements>,
    },
    Exit,
    UnconditionalJump { label: String },
}

#[derive(Clone, Debug)]
pub enum SimpleStatement {
    Assign { lhs: Expression, rhs: Expression },
    Send { lhs: Expression, rhs: Expression },
    Exchange { lhs: Expression, rhs: Expression },
    Expression { expression: Expression },

}

#[derive(Clone, Debug)]
pub enum Expression {
    Null,
    Float {
        value: f64,
    },
    Bool {
        value: bool,
    },
    Int {
        value: i64,
    },
    String {
        value: String,
    },
    Var {
        name: String,
    },
    Call {
        function: String,
        args: Vec<Box<Expression>>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Dereference,
    Not,
}
