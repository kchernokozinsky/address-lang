use common::location::Location;
use core::fmt;
use serde::{Deserialize, Serialize};

pub mod serializer;
pub mod visitor;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Located<T = ()> {
    pub l_location: Location,
    pub r_location: Location,
    pub node: T,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Algorithm {
    Body(Vec<FileLine>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statements {
    OneLineStatement(OneLineStatement),
    SimpleStatements(Vec<SimpleStatement>),
}

pub type OneLineStatement = Located<OneLineStatementKind>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimpleStatementKind {
    Import {
        labels: Vec<String>,
        path: Path,
        alias: Option<String>,
    },
    Del {
        rhs: Expression,
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

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Path {
    pub absolute: bool,
    pub ids: Vec<String>,
}

pub type Expression = Located<ExpressionKind>;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl PartialEq for ExpressionKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ExpressionKind::Null, ExpressionKind::Null) => true,
            (ExpressionKind::Float { value: v1 }, ExpressionKind::Float { value: v2 }) => {
                v1.to_bits() == v2.to_bits()
            }
            (ExpressionKind::Bool { value: v1 }, ExpressionKind::Bool { value: v2 }) => v1 == v2,
            (ExpressionKind::Int { value: v1 }, ExpressionKind::Int { value: v2 }) => v1 == v2,
            (ExpressionKind::String { value: s1 }, ExpressionKind::String { value: s2 }) => {
                s1 == s2
            }
            (ExpressionKind::Var { name: n1 }, ExpressionKind::Var { name: n2 }) => n1 == n2,
            (ExpressionKind::List { elements: e1 }, ExpressionKind::List { elements: e2 }) => {
                e1 == e2
            }
            (
                ExpressionKind::Call {
                    function: f1,
                    args: a1,
                },
                ExpressionKind::Call {
                    function: f2,
                    args: a2,
                },
            ) => f1 == f2 && a1 == a2,
            (
                ExpressionKind::UnaryOp { op: o1, expr: e1 },
                ExpressionKind::UnaryOp { op: o2, expr: e2 },
            ) => o1 == o2 && e1 == e2,
            (
                ExpressionKind::BinaryOp {
                    op: o1,
                    lhs: l1,
                    rhs: r1,
                },
                ExpressionKind::BinaryOp {
                    op: o2,
                    lhs: l2,
                    rhs: r2,
                },
            ) => o1 == o2 && l1 == l2 && r1 == r2,
            _ => false,
        }
    }
}

impl Eq for ExpressionKind {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Dereference,
    Minus,
    MultipleDereference(Box<Expression>),
    Not,
}
