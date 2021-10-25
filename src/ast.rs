use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOperator {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Datum {
    Int(i64),
    Bool(bool),
}

impl fmt::Display for Datum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Datum::Int(int_) => write!(f, "{:?}", int_),
            Datum::Bool(bool_) => write!(f, "{:?}", bool_),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Data {
        value: Datum,
    },
    // UnaryExpr {
    //     op: UnaryOperator,
    //     child: Box<Node>,
    // },
    BinaryExpr {
        op: BinaryOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Data { value } => write!(f, "{}", value),
            // Node::UnaryExpr { op, child } => write!(f, "{}{}", op, child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}
