pub mod visitor;

use core::fmt;

use common::location::Location;

#[derive(Debug, PartialEq, Clone)]
pub struct Located<T = ()> {
    pub l_location: Location,
    pub r_location: Location,
    pub node: T,
}

#[derive(Clone, Debug)]
pub enum Algorithm {
    Body(Vec<FileLine>),
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

pub type OneLineStatement = Located<OneLineStatementKind>;

#[derive(Clone, Debug)]
pub enum OneLineStatementKind {
    SubProgram {
        sp_name: Label,
        args: Vec<Box<Expression>>,
        label_to: Option<String>,
    },
    Loop {
        initial_value: Expression,
        step: Expression,
        last_value_or_condition: Expression,
        iterator: Expression,
        label_until: String,
        label_to: Option<String>,
    },
    Predicate {
        condition: Expression,
        if_true: Box<Statements>,
        if_false: Box<Statements>,
    },
    Exit,
    Return,
    UnconditionalJump {
        label: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label {
    pub identifier: String,
    pub mod_alias: Option<String>,
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mod_alias.as_ref() {
            Some(m) => write!(f, "{}::{}", m, self.identifier),
            None => write!(f, "{}", self.identifier),
        }
    }
}

pub type SimpleStatement = Located<SimpleStatementKind>;

#[derive(Clone, Debug)]
pub enum SimpleStatementKind {
    Import {
        labels: Vec<String>,
        path: Path,
        alias: Option<String>,
    },
    Assign {
        lhs: Expression,
        rhs: Expression,
    },
    Send {
        lhs: Expression,
        rhs: Expression,
    },
    Exchange {
        lhs: Expression,
        rhs: Expression,
    },
    Expression {
        expression: Expression,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path {
    pub absolute: bool,
    pub ids: Vec<String>,
}

pub type Expression = Located<ExpressionKind>;

#[derive(Clone, Debug)]
pub enum ExpressionKind {
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
    List {
        elements: Vec<Box<Expression>>,
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
    MultipleDereference(Box<Expression>),
    Not,
}
